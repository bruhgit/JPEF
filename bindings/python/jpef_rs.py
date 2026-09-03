"""
Python compatibility library for JPEF (Rust Core).
Zero-dependency ctypes wrapper over jpef.dll / libjpef.so / libjpef.dylib.
"""

from ctypes import (
    CDLL,
    c_char_p,
    c_void_p,
    c_bool,
    c_uint32,
    c_uint64,
    c_size_t,
    c_double,
)
from dataclasses import dataclass
import os
from pathlib import Path
from typing import List, Optional


def _find_lib() -> str:
    base = Path(__file__).resolve().parent
    candidates = [
        base / "../../target/release/jpef.dll",
        base / "../../target/release/libjpef.dll",
        base / "../../target/release/libjpef.so",
        base / "../../target/release/libjpef.dylib",
        base / "jpef.dll",
    ]
    for c in candidates:
        if c.is_file():
            return str(c.resolve())
    return "jpef.dll"


_lib = CDLL(_find_lib())

# FFI Signatures
_lib.jpef_version.restype = c_char_p
_lib.jpef_version.argtypes = []

_lib.jpef_config_new.restype = c_void_p
_lib.jpef_config_new.argtypes = []

_lib.jpef_config_free.restype = None
_lib.jpef_config_free.argtypes = [c_void_p]

_lib.jpef_config_set_jar_path.restype = None
_lib.jpef_config_set_jar_path.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_output_dir.restype = None
_lib.jpef_config_set_output_dir.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_app_name.restype = None
_lib.jpef_config_set_app_name.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_version.restype = None
_lib.jpef_config_set_version.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_company.restype = None
_lib.jpef_config_set_company.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_targets.restype = None
_lib.jpef_config_set_targets.argtypes = [c_void_p, c_uint32]

_lib.jpef_config_set_gui_mode.restype = None
_lib.jpef_config_set_gui_mode.argtypes = [c_void_p, c_bool]

_lib.jpef_config_set_icon_path.restype = None
_lib.jpef_config_set_icon_path.argtypes = [c_void_p, c_char_p]

_lib.jpef_config_set_jvm_heap.restype = None
_lib.jpef_config_set_jvm_heap.argtypes = [c_void_p, c_char_p, c_char_p]

_lib.jpef_config_add_jvm_arg.restype = None
_lib.jpef_config_add_jvm_arg.argtypes = [c_void_p, c_char_p]

_lib.jpef_convert.restype = c_void_p
_lib.jpef_convert.argtypes = [c_void_p]

_lib.jpef_result_free.restype = None
_lib.jpef_result_free.argtypes = [c_void_p]

_lib.jpef_result_is_success.restype = c_bool
_lib.jpef_result_is_success.argtypes = [c_void_p]

_lib.jpef_result_get_artifact_count.restype = c_size_t
_lib.jpef_result_get_artifact_count.argtypes = [c_void_p]

_lib.jpef_result_get_artifact_path.restype = c_char_p
_lib.jpef_result_get_artifact_path.argtypes = [c_void_p, c_size_t]

_lib.jpef_result_get_artifact_platform.restype = c_char_p
_lib.jpef_result_get_artifact_platform.argtypes = [c_void_p, c_size_t]

_lib.jpef_result_get_artifact_size.restype = c_uint64
_lib.jpef_result_get_artifact_size.argtypes = [c_void_p, c_size_t]

_lib.jpef_result_get_elapsed_seconds.restype = c_double
_lib.jpef_result_get_elapsed_seconds.argtypes = [c_void_p]

_lib.jpef_result_get_errors.restype = c_char_p
_lib.jpef_result_get_errors.argtypes = [c_void_p]

_lib.jpef_inspect.restype = c_void_p
_lib.jpef_inspect.argtypes = [c_char_p]

_lib.jpef_jar_info_free.restype = None
_lib.jpef_jar_info_free.argtypes = [c_void_p]

_lib.jpef_jar_info_get_main_class.restype = c_char_p
_lib.jpef_jar_info_get_main_class.argtypes = [c_void_p]

_lib.jpef_jar_info_get_min_java_version.restype = c_uint32
_lib.jpef_jar_info_get_min_java_version.argtypes = [c_void_p]

_lib.jpef_jar_info_is_runnable.restype = c_bool
_lib.jpef_jar_info_is_runnable.argtypes = [c_void_p]


TARGET_EXE = 1 << 0
TARGET_ELF = 1 << 1
TARGET_APP = 1 << 2
TARGET_ALL = TARGET_EXE | TARGET_ELF | TARGET_APP


@dataclass
class Artifact:
    platform: str
    path: Path
    size_bytes: int


@dataclass
class ConvertResult:
    success: bool
    elapsed_seconds: float
    artifacts: List[Artifact]
    errors: str


@dataclass
class JarInfo:
    main_class: Optional[str]
    min_java_version: int
    is_runnable: bool


def version() -> str:
    raw = _lib.jpef_version()
    return raw.decode("utf-8") if raw else "1.0.0"


def inspect(jar_path: str | Path) -> JarInfo:
    path_bytes = str(jar_path).encode("utf-8")
    info_ptr = _lib.jpef_inspect(path_bytes)
    if not info_ptr:
        raise ValueError(f"Failed to inspect JAR: {jar_path}")
    try:
        mc = _lib.jpef_jar_info_get_main_class(info_ptr)
        min_v = _lib.jpef_jar_info_get_min_java_version(info_ptr)
        runnable = _lib.jpef_jar_info_is_runnable(info_ptr)
        return JarInfo(
            main_class=mc.decode("utf-8") if mc else None,
            min_java_version=min_v,
            is_runnable=runnable,
        )
    finally:
        _lib.jpef_jar_info_free(info_ptr)


def convert(
    jar_path: str | Path,
    output_dir: str | Path = "dist",
    app_name: Optional[str] = None,
    version_str: str = "1.0.0.0",
    company: str = "JPEF",
    targets: List[str] = ("exe", "elf", "app"),
    is_gui: bool = True,
    icon_path: Optional[str | Path] = None,
    min_heap: Optional[str] = None,
    max_heap: Optional[str] = None,
    jvm_args: Optional[List[str]] = None,
) -> ConvertResult:
    cfg = _lib.jpef_config_new()
    try:
        _lib.jpef_config_set_jar_path(cfg, str(jar_path).encode("utf-8"))
        _lib.jpef_config_set_output_dir(cfg, str(output_dir).encode("utf-8"))
        if app_name:
            _lib.jpef_config_set_app_name(cfg, app_name.encode("utf-8"))
        _lib.jpef_config_set_version(cfg, version_str.encode("utf-8"))
        _lib.jpef_config_set_company(cfg, company.encode("utf-8"))

        flags = 0
        for t in targets:
            ts = t.lower()
            if "exe" in ts:
                flags |= TARGET_EXE
            if "elf" in ts:
                flags |= TARGET_ELF
            if "app" in ts:
                flags |= TARGET_APP
        _lib.jpef_config_set_targets(cfg, flags)
        _lib.jpef_config_set_gui_mode(cfg, is_gui)

        if icon_path:
            _lib.jpef_config_set_icon_path(cfg, str(icon_path).encode("utf-8"))

        if min_heap or max_heap:
            _lib.jpef_config_set_jvm_heap(
                cfg,
                (min_heap or "").encode("utf-8"),
                (max_heap or "").encode("utf-8"),
            )

        if jvm_args:
            for a in jvm_args:
                _lib.jpef_config_add_jvm_arg(cfg, a.encode("utf-8"))

        res = _lib.jpef_convert(cfg)
        if not res:
            raise RuntimeError("jpef_convert returned NULL")

        try:
            success = _lib.jpef_result_is_success(res)
            elapsed = _lib.jpef_result_get_elapsed_seconds(res)
            err_raw = _lib.jpef_result_get_errors(res)
            errors = err_raw.decode("utf-8") if err_raw else ""

            count = _lib.jpef_result_get_artifact_count(res)
            artifacts = []
            for i in range(count):
                plat = _lib.jpef_result_get_artifact_platform(res, i)
                p = _lib.jpef_result_get_artifact_path(res, i)
                sz = _lib.jpef_result_get_artifact_size(res, i)
                artifacts.append(Artifact(
                    platform=plat.decode("utf-8") if plat else "",
                    path=Path(p.decode("utf-8")) if p else Path(),
                    size_bytes=sz,
                ))

            return ConvertResult(
                success=success,
                elapsed_seconds=elapsed,
                artifacts=artifacts,
                errors=errors,
            )
        finally:
            _lib.jpef_result_free(res)
    finally:
        _lib.jpef_config_free(cfg)
