package main

import (
	"errors"
	"path/filepath"
	"syscall"
	"unsafe"
)

type TargetPlatform uint32

const (
	TargetExe TargetPlatform = 1 << 0
	TargetElf TargetPlatform = 1 << 1
	TargetApp TargetPlatform = 1 << 2
	TargetAll TargetPlatform = TargetExe | TargetElf | TargetApp
)

type Artifact struct {
	Platform  string
	Path      string
	SizeBytes uint64
}

type ConvertResult struct {
	Success        bool
	ElapsedSeconds float64
	Artifacts      []Artifact
	Errors         string
}

type JarInfo struct {
	MainClass      string
	MinJavaVersion uint32
	IsRunnable     bool
	Error          string
}

var (
	mod = syscall.NewLazyDLL("jpef.dll")

	procVersion               = mod.NewProc("jpef_version")
	procConfigNew             = mod.NewProc("jpef_config_new")
	procConfigFree            = mod.NewProc("jpef_config_free")
	procConfigSetJarPath      = mod.NewProc("jpef_config_set_jar_path")
	procConfigSetOutputDir    = mod.NewProc("jpef_config_set_output_dir")
	procConfigSetAppName      = mod.NewProc("jpef_config_set_app_name")
	procConfigSetVersion      = mod.NewProc("jpef_config_set_version")
	procConfigSetCompany      = mod.NewProc("jpef_config_set_company")
	procConfigSetTargets      = mod.NewProc("jpef_config_set_targets")
	procConfigSetGuiMode      = mod.NewProc("jpef_config_set_gui_mode")
	procConfigSetJvmHeap      = mod.NewProc("jpef_config_set_jvm_heap")
	procConfigAddJvmArg       = mod.NewProc("jpef_config_add_jvm_arg")
	procConvert               = mod.NewProc("jpef_convert")
	procResultFree            = mod.NewProc("jpef_result_free")
	procResultIsSuccess       = mod.NewProc("jpef_result_is_success")
	procResultGetCount        = mod.NewProc("jpef_result_get_artifact_count")
	procResultGetPath         = mod.NewProc("jpef_result_get_artifact_path")
	procResultGetPlatform     = mod.NewProc("jpef_result_get_artifact_platform")
	procResultGetSize         = mod.NewProc("jpef_result_get_artifact_size")
	procResultGetElapsed      = mod.NewProc("jpef_result_get_elapsed_seconds")
	procResultGetErrors       = mod.NewProc("jpef_result_get_errors")
	procInspect               = mod.NewProc("jpef_inspect")
	procJarInfoFree           = mod.NewProc("jpef_jar_info_free")
	procJarInfoGetMainClass   = mod.NewProc("jpef_jar_info_get_main_class")
	procJarInfoGetMinJavaVer  = mod.NewProc("jpef_jar_info_get_min_java_version")
	procJarInfoIsRunnable     = mod.NewProc("jpef_jar_info_is_runnable")
)

func goString(ptr uintptr) string {
	if ptr == 0 {
		return ""
	}
	var bytes []byte
	p := (*byte)(unsafe.Pointer(ptr))
	for *p != 0 {
		bytes = append(bytes, *p)
		ptr++
		p = (*byte)(unsafe.Pointer(ptr))
	}
	return string(bytes)
}

func cString(s string) *byte {
	b := append([]byte(s), 0)
	return &b[0]
}

func Version() string {
	r, _, _ := procVersion.Call()
	return goString(r)
}

func Inspect(jarPath string) (JarInfo, error) {
	ptr, _, _ := procInspect.Call(uintptr(unsafe.Pointer(cString(jarPath))))
	if ptr == 0 {
		return JarInfo{}, errors.New("failed to inspect JAR archive")
	}
	defer procJarInfoFree.Call(ptr)

	mcPtr, _, _ := procJarInfoGetMainClass.Call(ptr)
	minJava, _, _ := procJarInfoGetMinJavaVer.Call(ptr)
	runnable, _, _ := procJarInfoIsRunnable.Call(ptr)

	return JarInfo{
		MainClass:      goString(mcPtr),
		MinJavaVersion: uint32(minJava),
		IsRunnable:     runnable != 0,
	}, nil
}

type ConvertOptions struct {
	JarPath   string
	OutputDir string
	AppName   string
	Version   string
	Company   string
	Targets   TargetPlatform
	IsGui     bool
	MinHeap   string
	MaxHeap   string
	JvmArgs   []string
}

func Convert(opts ConvertOptions) (*ConvertResult, error) {
	cfg, _, _ := procConfigNew.Call()
	if cfg == 0 {
		return nil, errors.New("failed to allocate JpefConfig")
	}
	defer procConfigFree.Call(cfg)

	procConfigSetJarPath.Call(cfg, uintptr(unsafe.Pointer(cString(opts.JarPath))))

	if opts.OutputDir != "" {
		absOut, _ := filepath.Abs(opts.OutputDir)
		procConfigSetOutputDir.Call(cfg, uintptr(unsafe.Pointer(cString(absOut))))
	}
	if opts.AppName != "" {
		procConfigSetAppName.Call(cfg, uintptr(unsafe.Pointer(cString(opts.AppName))))
	}
	if opts.Targets == 0 {
		opts.Targets = TargetAll
	}
	procConfigSetTargets.Call(cfg, uintptr(opts.Targets))

	var isGuiNum uintptr = 0
	if opts.IsGui {
		isGuiNum = 1
	}
	procConfigSetGuiMode.Call(cfg, isGuiNum)

	if opts.MinHeap != "" || opts.MaxHeap != "" {
		procConfigSetJvmHeap.Call(
			cfg,
			uintptr(unsafe.Pointer(cString(opts.MinHeap))),
			uintptr(unsafe.Pointer(cString(opts.MaxHeap))),
		)
	}

	for _, arg := range opts.JvmArgs {
		procConfigAddJvmArg.Call(cfg, uintptr(unsafe.Pointer(cString(arg))))
	}

	res, _, _ := procConvert.Call(cfg)
	if res == 0 {
		return nil, errors.New("jpef_convert returned NULL")
	}
	defer procResultFree.Call(res)

	succ, _, _ := procResultIsSuccess.Call(res)
	count, _, _ := procResultGetCount.Call(res)
	errPtr, _, _ := procResultGetErrors.Call(res)

	artifacts := make([]Artifact, 0, count)
	for i := uintptr(0); i < count; i++ {
		platPtr, _, _ := procResultGetPlatform.Call(res, i)
		pathPtr, _, _ := procResultGetPath.Call(res, i)
		sz, _, _ := procResultGetSize.Call(res, i)

		artifacts = append(artifacts, Artifact{
			Platform:  goString(platPtr),
			Path:      goString(pathPtr),
			SizeBytes: uint64(sz),
		})
	}

	return &ConvertResult{
		Success:   succ != 0,
		Artifacts: artifacts,
		Errors:    goString(errPtr),
	}, nil
}
