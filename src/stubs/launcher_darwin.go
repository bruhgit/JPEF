/*
 * JPEF - Java Portable Executable Format
 * Native macOS Mach-O Launcher Stub (Go)
 */

package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"syscall"
)

var (
	AppName        = "JavaApp"
	DefaultJvmArgs = ""
	BundledJrePath = "jre"
)

func fileExists(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return !info.IsDir()
}

func findJava(bundleDir string) string {
	// 1. Check bundled JRE in Contents/PlugIns or Contents/Resources/jre
	pPlugins := filepath.Join(bundleDir, "Contents", "PlugIns", "jre", "Contents", "Home", "bin", "java")
	if fileExists(pPlugins) {
		return pPlugins
	}
	pResJre := filepath.Join(bundleDir, "Contents", "Resources", BundledJrePath, "bin", "java")
	if fileExists(pResJre) {
		return pResJre
	}

	// 2. Check /usr/libexec/java_home (official macOS Java discovery)
	if out, err := exec.Command("/usr/libexec/java_home").Output(); err == nil {
		home := strings.TrimSpace(string(out))
		if home != "" {
			p := filepath.Join(home, "bin", "java")
			if fileExists(p) {
				return p
			}
		}
	}

	// 3. Check JAVA_HOME
	if javaHome := os.Getenv("JAVA_HOME"); javaHome != "" {
		p := filepath.Join(javaHome, "bin", "java")
		if fileExists(p) {
			return p
		}
	}

	// 4. Check Homebrew and standard locations
	commonPaths := []string{
		"/opt/homebrew/bin/java",
		"/usr/local/bin/java",
		"/usr/bin/java",
	}
	for _, p := range commonPaths {
		if fileExists(p) {
			return p
		}
	}

	// Glob in /Library/Java/JavaVirtualMachines/*/Contents/Home/bin/java
	if matches, err := filepath.Glob("/Library/Java/JavaVirtualMachines/*/Contents/Home/bin/java"); err == nil {
		for _, m := range matches {
			if fileExists(m) {
				return m
			}
		}
	}

	return ""
}

func main() {
	selfPath, err := os.Executable()
	if err != nil {
		selfPath = os.Args[0]
	}
	selfPath, err = filepath.EvalSymlinks(selfPath)
	if err != nil {
		selfPath = os.Args[0]
	}
	selfPath, _ = filepath.Abs(selfPath)

	// In a .app bundle, selfPath is: AppName.app/Contents/MacOS/AppName
	macOSDir := filepath.Dir(selfPath)
	contentsDir := filepath.Dir(macOSDir)
	bundleDir := filepath.Dir(contentsDir)

	// Target JAR: either inside Resources/app.jar or selfPath (if standalone polyglot)
	targetJar := filepath.Join(contentsDir, "Resources", "app.jar")
	if !fileExists(targetJar) {
		// Look for any .jar in Resources
		if jars, _ := filepath.Glob(filepath.Join(contentsDir, "Resources", "*.jar")); len(jars) > 0 {
			targetJar = jars[0]
		} else {
			// Fallback: self is polyglot
			targetJar = selfPath
		}
	}

	javaPath := findJava(bundleDir)
	if javaPath == "" {
		// Use osascript to display native macOS alert dialog if in GUI mode
		msg := fmt.Sprintf("Java Runtime Environment was not found.\\n\\nApplication %s requires Java to run.\\nWould you like to open the download page?", AppName)
		script := fmt.Sprintf("display alert \"Java Required\" message \"%s\" buttons {\"Cancel\", \"Download\"} default button \"Download\"", msg)
		out, _ := exec.Command("osascript", "-e", script).Output()
		if strings.Contains(string(out), "Download") {
			exec.Command("open", "https://adoptium.net/").Start()
		}
		os.Exit(1)
	}

	var cmdArgs []string
	cmdArgs = append(cmdArgs, javaPath)

	// Pass macOS Dock properties
	cmdArgs = append(cmdArgs, fmt.Sprintf("-Xdock:name=%s", AppName))
	iconPath := filepath.Join(contentsDir, "Resources", "AppIcon.icns")
	if fileExists(iconPath) {
		cmdArgs = append(cmdArgs, fmt.Sprintf("-Xdock:icon=%s", iconPath))
	}

	if DefaultJvmArgs != "" {
		fields := strings.Fields(DefaultJvmArgs)
		cmdArgs = append(cmdArgs, fields...)
	}

	cmdArgs = append(cmdArgs, "-jar", targetJar)

	if len(os.Args) > 1 {
		cmdArgs = append(cmdArgs, os.Args[1:]...)
	}

	env := os.Environ()
	err = syscall.Exec(javaPath, cmdArgs, env)
	if err != nil {
		cmd := exec.Command(javaPath, cmdArgs[1:]...)
		cmd.Stdin = os.Stdin
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		if runErr := cmd.Run(); runErr != nil {
			if exitErr, ok := runErr.(*exec.ExitError); ok {
				os.Exit(exitErr.ExitCode())
			}
			os.Exit(1)
		}
	}
}
