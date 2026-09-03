/**
 * JavaScript / Node.js test example for JPEF
 */

const path = require('path');
const jpef = require('./index');

const sampleJar = path.resolve(__dirname, '../../test_sample.jar');

console.log('========================================');
console.log(` JPEF Node.js / TypeScript Binding v${jpef.version()}`);
console.log('========================================\n');

// 1. Inspect
console.log(`[1] Inspecting JAR: ${sampleJar}`);
const info = jpef.inspect(sampleJar);
console.log(`  Main-Class:   ${info.mainClass}`);
console.log(`  Min Java:     Java ${info.minJavaVersion}+`);
console.log(`  Runnable:     ${info.isRunnable ? 'Yes' : 'No'}\n`);

// 2. Convert
console.log('[2] Converting to .exe, .elf, and .app via Node.js...');
const result = jpef.convert({
  jarPath: sampleJar,
  outputDir: path.resolve(__dirname, '../../dist_node'),
  appName: 'SampleAppNode',
  version: '1.0.0.0',
  companyName: 'JPEF Node.js',
  targets: ['exe', 'elf', 'app'],
  isGui: false,
  minHeap: '128m',
  maxHeap: '512m',
  jvmArgs: ['-Dfile.encoding=UTF-8'],
});

if (result.success) {
  console.log(`\n[SUCCESS] Generated ${result.artifacts.length} artifact(s) in ${result.elapsedSeconds.toFixed(2)}s:`);
  for (const art of result.artifacts) {
    console.log(`  - [${art.platform}] ${art.path} (${(art.sizeBytes / (1024 * 1024)).toFixed(2)} MB)`);
  }
} else {
  console.error(`\n[FAILED] Conversion error: ${result.errors}`);
  process.exit(1);
}
