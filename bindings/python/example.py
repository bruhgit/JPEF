"""
Example Python script using the Rust-powered JPEF library via ctypes.
"""

from pathlib import Path
import sys
import jpef_rs

sample_jar = Path(__file__).resolve().parent / "../../test_sample.jar"

print("========================================")
print(f" JPEF Python (Rust Core) Binding v{jpef_rs.version()}")
print("========================================\n")

# 1. Inspect
print(f"[1] Inspecting JAR: {sample_jar}")
info = jpef_rs.inspect(sample_jar)
print(f"  Main-Class:   {info.main_class}")
print(f"  Min Java:     Java {info.min_java_version}+")
print(f"  Runnable:     {'Yes' if info.is_runnable else 'No'}\n")

# 2. Convert
print("[2] Converting to .exe, .elf, and .app via Python...")
out_dir = Path(__file__).resolve().parent / "../../dist_py"
res = jpef_rs.convert(
    jar_path=sample_jar,
    output_dir=out_dir,
    app_name="SampleAppPython",
    targets=["exe", "elf", "app"],
    is_gui=False,
    min_heap="128m",
    max_heap="512m",
    jvm_args=["-Dfile.encoding=UTF-8"],
)

if res.success:
    print(f"\n[SUCCESS] Generated {len(res.artifacts)} artifact(s) in {res.elapsed_seconds:.2f}s:")
    for art in res.artifacts:
        print(f"  - [{art.platform}] {art.path} ({art.size_bytes / (1024*1024):.2f} MB)")
else:
    print(f"\n[FAILED] Conversion failed: {res.errors}")
    sys.exit(1)
