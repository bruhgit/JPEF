/*
 * JPEF - Java Portable Executable Format
 * Native Windows PE Launcher Stub (C / Win32)
 */

#ifndef UNICODE
#define UNICODE
#endif
#ifndef _UNICODE
#define _UNICODE
#endif
#define WIN32_LEAN_AND_MEAN

#include <windows.h>
#include <shellapi.h>
#include <shlwapi.h>
#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

#ifdef _MSC_VER
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "shlwapi.lib")
#endif

/* Embedded configuration placeholders or defaults */
#ifndef APP_NAME
#define APP_NAME L"Java Application"
#endif

#ifndef DEFAULT_JVM_ARGS
#define DEFAULT_JVM_ARGS L""
#endif

#ifndef BUNDLED_JRE_PATH
#define BUNDLED_JRE_PATH L"jre"
#endif

#define JAVA_DOWNLOAD_URL L"https://adoptium.net/"

/* Helper to check if a file exists */
static BOOL FileExists(const wchar_t *path) {
    DWORD dwAttrib = GetFileAttributesW(path);
    return (dwAttrib != INVALID_FILE_ATTRIBUTES && !(dwAttrib & FILE_ATTRIBUTE_DIRECTORY));
}

/* Helper to check if a directory exists */
static BOOL DirectoryExists(const wchar_t *path) {
    DWORD dwAttrib = GetFileAttributesW(path);
    return (dwAttrib != INVALID_FILE_ATTRIBUTES && (dwAttrib & FILE_ATTRIBUTE_DIRECTORY));
}

/* Find Java executable */
static BOOL FindJava(const wchar_t *exeDir, wchar_t *outJavaPath, size_t maxLen, BOOL preferGui) {
    const wchar_t *targetBinary = preferGui ? L"javaw.exe" : L"java.exe";
    const wchar_t *fallbackBinary = preferGui ? L"java.exe" : L"javaw.exe";

    /* 1. Check bundled JRE relative to current executable */
    if (BUNDLED_JRE_PATH[0] != L'\0') {
        _snwprintf(outJavaPath, maxLen, L"%s\\%s\\bin\\%s", exeDir, BUNDLED_JRE_PATH, targetBinary);
        if (FileExists(outJavaPath)) return TRUE;
        _snwprintf(outJavaPath, maxLen, L"%s\\%s\\bin\\%s", exeDir, BUNDLED_JRE_PATH, fallbackBinary);
        if (FileExists(outJavaPath)) return TRUE;

        /* Also check "runtime" */
        _snwprintf(outJavaPath, maxLen, L"%s\\runtime\\bin\\%s", exeDir, targetBinary);
        if (FileExists(outJavaPath)) return TRUE;
    }

    /* 2. Check JAVA_HOME environment variable */
    wchar_t envJavaHome[MAX_PATH];
    DWORD envLen = GetEnvironmentVariableW(L"JAVA_HOME", envJavaHome, MAX_PATH);
    if (envLen > 0 && envLen < MAX_PATH) {
        _snwprintf(outJavaPath, maxLen, L"%s\\bin\\%s", envJavaHome, targetBinary);
        if (FileExists(outJavaPath)) return TRUE;
        _snwprintf(outJavaPath, maxLen, L"%s\\bin\\%s", envJavaHome, fallbackBinary);
        if (FileExists(outJavaPath)) return TRUE;
    }

    /* 3. Check PATH */
    wchar_t searchPath[MAX_PATH];
    if (SearchPathW(NULL, targetBinary, NULL, MAX_PATH, searchPath, NULL) > 0) {
        wcsncpy(outJavaPath, searchPath, maxLen);
        return TRUE;
    }
    if (SearchPathW(NULL, fallbackBinary, NULL, MAX_PATH, searchPath, NULL) > 0) {
        wcsncpy(outJavaPath, searchPath, maxLen);
        return TRUE;
    }

    /* 4. Check Windows Registry (HKLM & HKCU) */
    HKEY rootKeys[] = { HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER };
    const wchar_t *subKeys[] = {
        L"SOFTWARE\\JavaSoft\\JDK",
        L"SOFTWARE\\JavaSoft\\Java Runtime Environment",
        L"SOFTWARE\\Eclipse Adoptium\\JDK",
        L"SOFTWARE\\Eclipse Adoptium\\JRE"
    };

    for (int r = 0; r < 2; r++) {
        for (int k = 0; k < 4; k++) {
            HKEY hKey;
            if (RegOpenKeyExW(rootKeys[r], subKeys[k], 0, KEY_READ, &hKey) == ERROR_SUCCESS) {
                wchar_t currentVer[64] = {0};
                DWORD verSize = sizeof(currentVer);
                if (RegQueryValueExW(hKey, L"CurrentVersion", NULL, NULL, (LPBYTE)currentVer, &verSize) == ERROR_SUCCESS) {
                    HKEY hVerKey;
                    if (RegOpenKeyExW(hKey, currentVer, 0, KEY_READ, &hVerKey) == ERROR_SUCCESS) {
                        wchar_t javaHome[MAX_PATH] = {0};
                        DWORD homeSize = sizeof(javaHome);
                        if (RegQueryValueExW(hVerKey, L"JavaHome", NULL, NULL, (LPBYTE)javaHome, &homeSize) == ERROR_SUCCESS) {
                            _snwprintf(outJavaPath, maxLen, L"%s\\bin\\%s", javaHome, targetBinary);
                            RegCloseKey(hVerKey);
                            RegCloseKey(hKey);
                            if (FileExists(outJavaPath)) return TRUE;
                        }
                        RegCloseKey(hVerKey);
                    }
                }
                RegCloseKey(hKey);
            }
        }
    }

    /* 5. Scan common directories in Program Files */
    const wchar_t *basePaths[] = {
        L"C:\\Program Files\\Java",
        L"C:\\Program Files\\Eclipse Adoptium",
        L"C:\\Program Files\\BellSoft",
        L"C:\\Program Files\\Microsoft",
        L"C:\\Program Files (x86)\\Java"
    };

    for (int b = 0; b < 5; b++) {
        if (!DirectoryExists(basePaths[b])) continue;
        wchar_t findPattern[MAX_PATH];
        _snwprintf(findPattern, MAX_PATH, L"%s\\*", basePaths[b]);
        WIN32_FIND_DATAW findData;
        HANDLE hFind = FindFirstFileW(findPattern, &findData);
        if (hFind != INVALID_HANDLE_VALUE) {
            do {
                if ((findData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) &&
                    wcscmp(findData.cFileName, L".") != 0 &&
                    wcscmp(findData.cFileName, L"..") != 0) {
                    _snwprintf(outJavaPath, maxLen, L"%s\\%s\\bin\\%s", basePaths[b], findData.cFileName, targetBinary);
                    if (FileExists(outJavaPath)) {
                        FindClose(hFind);
                        return TRUE;
                    }
                }
            } while (FindNextFileW(hFind, &findData));
            FindClose(hFind);
        }
    }

    return FALSE;
}

#ifdef JPEF_GUI_MODE
int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR pCmdLine, int nCmdShow)
#else
int wmain(int argc, wchar_t *argv[])
#endif
{
    wchar_t selfExePath[MAX_PATH];
    if (GetModuleFileNameW(NULL, selfExePath, MAX_PATH) == 0) {
        return 1;
    }

    /* Directory containing this executable */
    wchar_t exeDir[MAX_PATH];
    wcsncpy(exeDir, selfExePath, MAX_PATH);
    PathRemoveFileSpecW(exeDir);

#ifdef JPEF_GUI_MODE
    BOOL preferGui = TRUE;
#else
    BOOL preferGui = FALSE;
#endif

    wchar_t javaPath[MAX_PATH];
    if (!FindJava(exeDir, javaPath, MAX_PATH, preferGui)) {
#ifdef JPEF_GUI_MODE
        wchar_t errorMsg[1024];
        _snwprintf(errorMsg, 1024,
            L"Java Runtime Environment was not found on this computer.\n\n"
            L"Application: %s\n"
            L"To run this application, Java 8 or newer is required.\n\n"
            L"Would you like to visit the Java download page now?",
            APP_NAME
        );
        int choice = MessageBoxW(NULL, errorMsg, APP_NAME, MB_ICONERROR | MB_YESNO);
        if (choice == IDYES) {
            ShellExecuteW(NULL, L"open", JAVA_DOWNLOAD_URL, NULL, NULL, SW_SHOWNORMAL);
        }
#else
        fwprintf(stderr, L"[JPEF Error] Java Runtime Environment not found.\n");
        fwprintf(stderr, L"Please install Java 8+ or set JAVA_HOME. Download: %s\n", JAVA_DOWNLOAD_URL);
#endif
        return 1;
    }

    /* Construct complete command line */
    /* Size: javaPath + JVM_ARGS + " -jar " + selfExePath + passed arguments + safety */
    const wchar_t *cmdLineToPass = GetCommandLineW();

    /* Skip executable name in GetCommandLineW */
    const wchar_t *argsStart = cmdLineToPass;
    if (*argsStart == L'"') {
        argsStart++;
        while (*argsStart && *argsStart != L'"') argsStart++;
        if (*argsStart == L'"') argsStart++;
    } else {
        while (*argsStart && *argsStart != L' ' && *argsStart != L'\t') argsStart++;
    }
    while (*argsStart == L' ' || *argsStart == L'\t') argsStart++;

    /* Allocate buffer for command line */
    size_t fullCmdLen = wcslen(javaPath) + wcslen(DEFAULT_JVM_ARGS) + wcslen(selfExePath) + wcslen(argsStart) + 128;
    wchar_t *fullCmd = (wchar_t *)malloc(fullCmdLen * sizeof(wchar_t));
    if (!fullCmd) return 1;

    if (wcslen(DEFAULT_JVM_ARGS) > 0) {
        _snwprintf(fullCmd, fullCmdLen, L"\"%s\" %s -jar \"%s\" %s", javaPath, DEFAULT_JVM_ARGS, selfExePath, argsStart);
    } else {
        _snwprintf(fullCmd, fullCmdLen, L"\"%s\" -jar \"%s\" %s", javaPath, selfExePath, argsStart);
    }

    /* Start child process */
    STARTUPINFOW si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    DWORD creationFlags = 0;
#ifdef JPEF_GUI_MODE
    /* In GUI mode, ensure no console window flickers if fallback java.exe is picked */
    creationFlags = CREATE_NO_WINDOW;
#endif

    BOOL success = CreateProcessW(
        NULL,
        fullCmd,
        NULL,
        NULL,
        TRUE,
        creationFlags,
        NULL,
        NULL,
        &si,
        &pi
    );

    free(fullCmd);

    if (!success) {
#ifdef JPEF_GUI_MODE
        MessageBoxW(NULL, L"Failed to launch Java process.", APP_NAME, MB_ICONERROR | MB_OK);
#else
        fwprintf(stderr, L"[JPEF Error] Failed to launch Java process (Error %lu).\n", GetLastError());
#endif
        return 1;
    }

    /* Wait for child process to terminate */
    WaitForSingleObject(pi.hProcess, INFINITE);
    DWORD exitCode = 0;
    GetExitCodeProcess(pi.hProcess, &exitCode);

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    return (int)exitCode;
}
