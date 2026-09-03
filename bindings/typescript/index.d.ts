/**
 * TypeScript definitions for JPEF (Java Portable Executable Format)
 */

export type TargetPlatform = 'exe' | 'elf' | 'app';

export interface ConvertOptions {
  jarPath: string;
  outputDir?: string;
  appName?: string;
  version?: string;
  companyName?: string;
  targets?: TargetPlatform[];
  isGui?: boolean;
  iconPath?: string;
  minHeap?: string;
  maxHeap?: string;
  jvmArgs?: string[];
}

export interface Artifact {
  platform: string;
  path: string;
  sizeBytes: number;
}

export interface ConvertResult {
  success: boolean;
  elapsedSeconds: number;
  artifacts: Artifact[];
  errors: string;
}

export interface JarInfo {
  mainClass: string | null;
  minJavaVersion: number;
  isRunnable: boolean;
  error: string | null;
}

export declare function version(): string;
export declare function inspect(jarPath: string): JarInfo;
export declare function convert(options: ConvertOptions): ConvertResult;

export default {
  version,
  inspect,
  convert,
};
