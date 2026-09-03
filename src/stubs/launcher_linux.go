/*
 * JPEF - Java Portable Executable Format
 * Native Linux ELF Launcher Stub (Go / Static ELF)
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

// Defaults that can be overridden at build time or metadata
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

func findJava(exeDir string) string {
	// 1. Check bundled JRE relative to binary
	if BundledJrePath != "" {
		p1 := filepath.Join(exeDir, BundledJrePath, "bin", "java")
		if fileExists(p1) {
			return p1
		}
		p2 := filepath.Join(exeDir, "runtime", "bin", "java")
		if fileExists(p2) {
			return p2
		}
	}

	// 2. Check JAVA_HOME environment variable
	if javaHome := os.Getenv("JAVA_HOME"); javaHome != "" {
		p := filepath.Join(javaHome, "bin", "java")
		if fileExists(p) {
			return p
		}
	}

	// 3. Check PATH
	if p, err := exec.LookPath("java"); err == nil {
		return p
	}

	// 4. Common standard Linux JVM directories
	standardPaths := []string{
		"/usr/bin/java",
		"/usr/local/bin/java",
		"/etc/alternatives/java",
		"/usr/lib/jvm/default-java/bin/java",
	}
	for _, p := range standardPaths {
		if fileExists(p) {
			return p
		}
	}

	// Glob in /usr/lib/jvm/*/bin/java
	if matches, err := filepath.Glob("/usr/lib/jvm/*/bin/java"); err == nil {
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
	exeDir := filepath.Dir(selfPath)

	javaPath := findJava(exeDir)
	if javaPath == "" {
		fmt.Fprintf(os.Stderr, "\033[1;31m[JPEF Error]\033[0m Java Runtime Environment was not found on this system.\n")
		fmt.Fprintf(os.Stderr, "Application: %s\n", AppName)
		fmt.Fprintf(os.Stderr, "Please install Java (JRE/JDK 8+) using your package manager:\n")
		fmt.Fprintf(os.Stderr, "  Debian/Ubuntu: sudo apt install default-jre\n")
		fmt.Fprintf(os.Stderr, "  Fedora/RHEL:   sudo dnf install java-latest-openjdk\n")
		fmt.Fprintf(os.Stderr, "  Arch Linux:    sudo pacman -S jre-openjdk\n")
		fmt.Fprintf(os.Stderr, "Or download from: https://adoptium.net/\n")
		os.Exit(1)
	}

	// Construct argument list
	var cmdArgs []string
	cmdArgs = append(cmdArgs, javaPath)

	if DefaultJvmArgs != "" {
		fields := strings.Fields(DefaultJvmArgs)
		cmdArgs = append(cmdArgs, fields...)
	}

	cmdArgs = append(cmdArgs, "-jar", selfPath)

	// Forward all user arguments (skip argv[0])
	if len(os.Args) > 1 {
		cmdArgs = append(cmdArgs, os.Args[1:]...)
	}

	// Replace current process with Java using syscall.Exec for 0-overhead native execution
	env := os.Environ()
	err = syscall.Exec(javaPath, cmdArgs, env)
	if err != nil {
		// Fallback to exec.Command if syscall.Exec fails
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
