import jpef.Jpef;
import java.nio.file.Path;
import java.util.List;

public class Example {
    public static void main(String[] args) {
        Path sampleJar = Path.of("test_sample.jar").toAbsolutePath();

        System.out.println("========================================");
        System.out.println(" JPEF Java (FFM API) Binding v" + Jpef.version());
        System.out.println("========================================\n");

        // 1. Inspect
        System.out.println("[1] Inspecting JAR: " + sampleJar);
        Jpef.JarInfo info = Jpef.inspect(sampleJar.toString());
        System.out.println("  Main-Class:   " + info.mainClass());
        System.out.println("  Min Java:     Java " + info.minJavaVersion() + "+");
        System.out.println("  Runnable:     " + (info.isRunnable() ? "Yes" : "No") + "\n");

        // 2. Convert
        System.out.println("[2] Converting to .exe, .elf, and .app via Java...");
        Path outDir = Path.of("dist_java").toAbsolutePath();
        Jpef.ConvertResult result = Jpef.convert(
            sampleJar.toString(),
            outDir.toString(),
            "SampleAppJava",
            false,
            List.of("-Dfile.encoding=UTF-8")
        );

        if (result.success()) {
            System.out.printf("\n[SUCCESS] Generated %d artifact(s) in %.2fs:\n",
                result.artifacts().size(), result.elapsedSeconds());
            for (Jpef.Artifact art : result.artifacts()) {
                System.out.printf("  - [%s] %s (%.2f MB)\n",
                    art.platform(), art.path(), art.sizeBytes() / (1024.0 * 1024.0));
            }
        } else {
            System.err.println("\n[FAILED] Conversion failed: " + result.errors());
            System.exit(1);
        }
    }
}
