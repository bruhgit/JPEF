package jpef;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/**
 * Java 21 Foreign Function & Memory API (Panama) binding for JPEF Rust core.
 */
public class Jpef {
    private static final SymbolLookup lookup;
    private static final Linker linker = Linker.nativeLinker();

    // Method handles
    private static final MethodHandle h_version;
    private static final MethodHandle h_config_new;
    private static final MethodHandle h_config_free;
    private static final MethodHandle h_config_set_jar_path;
    private static final MethodHandle h_config_set_output_dir;
    private static final MethodHandle h_config_set_app_name;
    private static final MethodHandle h_config_set_targets;
    private static final MethodHandle h_config_set_gui_mode;
    private static final MethodHandle h_config_set_jvm_heap;
    private static final MethodHandle h_config_add_jvm_arg;
    private static final MethodHandle h_convert;
    private static final MethodHandle h_result_free;
    private static final MethodHandle h_result_is_success;
    private static final MethodHandle h_result_get_count;
    private static final MethodHandle h_result_get_path;
    private static final MethodHandle h_result_get_platform;
    private static final MethodHandle h_result_get_size;
    private static final MethodHandle h_result_get_elapsed;
    private static final MethodHandle h_result_get_errors;
    private static final MethodHandle h_inspect;
    private static final MethodHandle h_jar_info_free;
    private static final MethodHandle h_jar_info_get_main_class;
    private static final MethodHandle h_jar_info_get_min_java;
    private static final MethodHandle h_jar_info_is_runnable;

    static {
        Path dllPath = Path.of("../../target/release/jpef.dll").toAbsolutePath();
        if (!dllPath.toFile().exists()) {
            dllPath = Path.of("target/release/jpef.dll").toAbsolutePath();
        }
        System.load(dllPath.toString());
        lookup = SymbolLookup.loaderLookup();

        try {
            h_version = linker.downcallHandle(lookup.find("jpef_version").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS));
            h_config_new = linker.downcallHandle(lookup.find("jpef_config_new").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS));
            h_config_free = linker.downcallHandle(lookup.find("jpef_config_free").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            h_config_set_jar_path = linker.downcallHandle(lookup.find("jpef_config_set_jar_path").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_config_set_output_dir = linker.downcallHandle(lookup.find("jpef_config_set_output_dir").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_config_set_app_name = linker.downcallHandle(lookup.find("jpef_config_set_app_name").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_config_set_targets = linker.downcallHandle(lookup.find("jpef_config_set_targets").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
            h_config_set_gui_mode = linker.downcallHandle(lookup.find("jpef_config_set_gui_mode").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.JAVA_BOOLEAN));
            h_config_set_jvm_heap = linker.downcallHandle(lookup.find("jpef_config_set_jvm_heap").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_config_add_jvm_arg = linker.downcallHandle(lookup.find("jpef_config_add_jvm_arg").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

            h_convert = linker.downcallHandle(lookup.find("jpef_convert").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_result_free = linker.downcallHandle(lookup.find("jpef_result_free").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            h_result_is_success = linker.downcallHandle(lookup.find("jpef_result_is_success").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS));
            h_result_get_count = linker.downcallHandle(lookup.find("jpef_result_get_artifact_count").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS));
            h_result_get_path = linker.downcallHandle(lookup.find("jpef_result_get_artifact_path").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));
            h_result_get_platform = linker.downcallHandle(lookup.find("jpef_result_get_artifact_platform").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));
            h_result_get_size = linker.downcallHandle(lookup.find("jpef_result_get_artifact_size").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));
            h_result_get_elapsed = linker.downcallHandle(lookup.find("jpef_result_get_elapsed_seconds").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_DOUBLE, ValueLayout.ADDRESS));
            h_result_get_errors = linker.downcallHandle(lookup.find("jpef_result_get_errors").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

            h_inspect = linker.downcallHandle(lookup.find("jpef_inspect").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_jar_info_free = linker.downcallHandle(lookup.find("jpef_jar_info_free").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            h_jar_info_get_main_class = linker.downcallHandle(lookup.find("jpef_jar_info_get_main_class").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            h_jar_info_get_min_java = linker.downcallHandle(lookup.find("jpef_jar_info_get_min_java_version").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));
            h_jar_info_is_runnable = linker.downcallHandle(lookup.find("jpef_jar_info_is_runnable").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS));
        } catch (Exception e) {
            throw new RuntimeException("Failed to bind JPEF functions: " + e.getMessage(), e);
        }
    }

    public record Artifact(String platform, String path, long sizeBytes) {}

    public record ConvertResult(boolean success, double elapsedSeconds, List<Artifact> artifacts, String errors) {}

    public record JarInfo(String mainClass, int minJavaVersion, boolean isRunnable) {}

    private static String readCString(MemorySegment ptr) {
        if (ptr == null || ptr.equals(MemorySegment.NULL) || ptr.address() == 0) {
            return null;
        }
        return ptr.reinterpret(Long.MAX_VALUE).getUtf8String(0);
    }

    public static String version() {
        try {
            MemorySegment res = (MemorySegment) h_version.invoke();
            String v = readCString(res);
            return v != null ? v : "1.0.0";
        } catch (Throwable t) {
            return "1.0.0";
        }
    }

    public static JarInfo inspect(String jarPath) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment cPath = arena.allocateUtf8String(jarPath);
            MemorySegment info = (MemorySegment) h_inspect.invoke(cPath);
            if (info.equals(MemorySegment.NULL)) {
                return new JarInfo(null, 0, false);
            }
            try {
                MemorySegment mcPtr = (MemorySegment) h_jar_info_get_main_class.invoke(info);
                String mainClass = readCString(mcPtr);
                int minJava = (int) h_jar_info_get_min_java.invoke(info);
                boolean runnable = (boolean) h_jar_info_is_runnable.invoke(info);
                return new JarInfo(mainClass, minJava, runnable);
            } finally {
                h_jar_info_free.invoke(info);
            }
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    public static ConvertResult convert(String jarPath, String outputDir, String appName, boolean isGui, List<String> jvmArgs) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment cfg = (MemorySegment) h_config_new.invoke();
            try {
                h_config_set_jar_path.invoke(cfg, arena.allocateUtf8String(jarPath));
                if (outputDir != null) h_config_set_output_dir.invoke(cfg, arena.allocateUtf8String(outputDir));
                if (appName != null) h_config_set_app_name.invoke(cfg, arena.allocateUtf8String(appName));
                h_config_set_targets.invoke(cfg, 7); // 1|2|4 = All
                h_config_set_gui_mode.invoke(cfg, isGui);

                if (jvmArgs != null) {
                    for (String arg : jvmArgs) {
                        h_config_add_jvm_arg.invoke(cfg, arena.allocateUtf8String(arg));
                    }
                }

                MemorySegment res = (MemorySegment) h_convert.invoke(cfg);
                if (res.equals(MemorySegment.NULL)) {
                    throw new RuntimeException("jpef_convert returned NULL");
                }

                try {
                    boolean success = (boolean) h_result_is_success.invoke(res);
                    double elapsed = (double) h_result_get_elapsed.invoke(res);
                    long count = (long) h_result_get_count.invoke(res);
                    MemorySegment errPtr = (MemorySegment) h_result_get_errors.invoke(res);
                    String errors = readCString(errPtr);
                    if (errors == null) errors = "";

                    List<Artifact> artifacts = new ArrayList<>();
                    for (long i = 0; i < count; i++) {
                        MemorySegment pPlat = (MemorySegment) h_result_get_platform.invoke(res, i);
                        MemorySegment pPath = (MemorySegment) h_result_get_path.invoke(res, i);
                        long sz = (long) h_result_get_size.invoke(res, i);
                        artifacts.add(new Artifact(
                            readCString(pPlat),
                            readCString(pPath),
                            sz
                        ));
                    }
                    return new ConvertResult(success, elapsed, artifacts, errors);
                } finally {
                    h_result_free.invoke(res);
                }
            } finally {
                h_config_free.invoke(cfg);
            }
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }
}
