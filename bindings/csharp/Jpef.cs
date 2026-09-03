using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;

namespace Jpef
{
    [Flags]
    public enum TargetPlatform : uint
    {
        Exe = 1 << 0,
        Elf = 1 << 1,
        App = 1 << 2,
        All = Exe | Elf | App
    }

    public class Artifact
    {
        public string Platform { get; set; } = string.Empty;
        public string Path { get; set; } = string.Empty;
        public ulong SizeBytes { get; set; }
    }

    public class ConvertResult
    {
        public bool Success { get; set; }
        public double ElapsedSeconds { get; set; }
        public string Errors { get; set; } = string.Empty;
        public List<Artifact> Artifacts { get; set; } = new();
    }

    public class JarInfo
    {
        public string? MainClass { get; set; }
        public uint MinJavaVersion { get; set; }
        public bool IsRunnable { get; set; }
        public string? Error { get; set; }
    }

    internal static class NativeMethods
    {
        private const string LibName = "jpef";

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_version();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_config_new();

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_free(IntPtr config);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_jar_path(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string jarPath);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_output_dir(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string outputDir);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_app_name(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string appName);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_version(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string version);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_company(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string company);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_targets(IntPtr config, uint flags);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_gui_mode(IntPtr config, [MarshalAs(UnmanagedType.I1)] bool isGui);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_set_jvm_heap(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string minHeap, [MarshalAs(UnmanagedType.LPUTF8Str)] string maxHeap);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_config_add_jvm_arg(IntPtr config, [MarshalAs(UnmanagedType.LPUTF8Str)] string arg);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_convert(IntPtr config);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_result_free(IntPtr result);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool jpef_result_is_success(IntPtr result);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern nuint jpef_result_get_artifact_count(IntPtr result);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_result_get_artifact_path(IntPtr result, nuint index);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_result_get_artifact_platform(IntPtr result, nuint index);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong jpef_result_get_artifact_size(IntPtr result, nuint index);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern double jpef_result_get_elapsed_seconds(IntPtr result);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_result_get_errors(IntPtr result);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_inspect([MarshalAs(UnmanagedType.LPUTF8Str)] string jarPath);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void jpef_jar_info_free(IntPtr info);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr jpef_jar_info_get_main_class(IntPtr info);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        public static extern uint jpef_jar_info_get_min_java_version(IntPtr info);

        [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool jpef_jar_info_is_runnable(IntPtr info);
    }

    public static class JpefClient
    {
        public static string Version()
        {
            IntPtr ptr = NativeMethods.jpef_version();
            return Marshal.PtrToStringUTF8(ptr) ?? "1.0.0";
        }

        public static JarInfo Inspect(string jarPath)
        {
            IntPtr ptr = NativeMethods.jpef_inspect(jarPath);
            if (ptr == IntPtr.Zero)
            {
                return new JarInfo { Error = "Failed to inspect JAR" };
            }

            try
            {
                IntPtr mcPtr = NativeMethods.jpef_jar_info_get_main_class(ptr);
                return new JarInfo
                {
                    MainClass = mcPtr != IntPtr.Zero ? Marshal.PtrToStringUTF8(mcPtr) : null,
                    MinJavaVersion = NativeMethods.jpef_jar_info_get_min_java_version(ptr),
                    IsRunnable = NativeMethods.jpef_jar_info_is_runnable(ptr)
                };
            }
            finally
            {
                NativeMethods.jpef_jar_info_free(ptr);
            }
        }

        public static ConvertResult Convert(
            string jarPath,
            string outputDir = "dist",
            string? appName = null,
            TargetPlatform targets = TargetPlatform.All,
            bool isGui = true,
            string? minHeap = null,
            string? maxHeap = null,
            IEnumerable<string>? jvmArgs = null)
        {
            IntPtr cfg = NativeMethods.jpef_config_new();
            try
            {
                NativeMethods.jpef_config_set_jar_path(cfg, jarPath);
                NativeMethods.jpef_config_set_output_dir(cfg, outputDir);
                if (!string.IsNullOrEmpty(appName))
                    NativeMethods.jpef_config_set_app_name(cfg, appName);

                NativeMethods.jpef_config_set_targets(cfg, (uint)targets);
                NativeMethods.jpef_config_set_gui_mode(cfg, isGui);

                if (!string.IsNullOrEmpty(minHeap) || !string.IsNullOrEmpty(maxHeap))
                    NativeMethods.jpef_config_set_jvm_heap(cfg, minHeap ?? "", maxHeap ?? "");

                if (jvmArgs != null)
                {
                    foreach (var arg in jvmArgs)
                        NativeMethods.jpef_config_add_jvm_arg(cfg, arg);
                }

                IntPtr res = NativeMethods.jpef_convert(cfg);
                if (res == IntPtr.Zero)
                    throw new InvalidOperationException("jpef_convert returned NULL pointer");

                try
                {
                    bool success = NativeMethods.jpef_result_is_success(res);
                    double elapsed = NativeMethods.jpef_result_get_elapsed_seconds(res);
                    IntPtr errPtr = NativeMethods.jpef_result_get_errors(res);
                    string errors = errPtr != IntPtr.Zero ? (Marshal.PtrToStringUTF8(errPtr) ?? "") : "";

                    nuint count = NativeMethods.jpef_result_get_artifact_count(res);
                    var artifacts = new List<Artifact>();

                    for (nuint i = 0; i < count; i++)
                    {
                        IntPtr platPtr = NativeMethods.jpef_result_get_artifact_platform(res, i);
                        IntPtr pathPtr = NativeMethods.jpef_result_get_artifact_path(res, i);
                        ulong sz = NativeMethods.jpef_result_get_artifact_size(res, i);

                        artifacts.Add(new Artifact
                        {
                            Platform = platPtr != IntPtr.Zero ? (Marshal.PtrToStringUTF8(platPtr) ?? "") : "",
                            Path = pathPtr != IntPtr.Zero ? (Marshal.PtrToStringUTF8(pathPtr) ?? "") : "",
                            SizeBytes = sz
                        });
                    }

                    return new ConvertResult
                    {
                        Success = success,
                        ElapsedSeconds = elapsed,
                        Errors = errors,
                        Artifacts = artifacts
                    };
                }
                finally
                {
                    NativeMethods.jpef_result_free(res);
                }
            }
            finally
            {
                NativeMethods.jpef_config_free(cfg);
            }
        }
    }
}
