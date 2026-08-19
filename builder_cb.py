import subprocess
import maturin
from pathlib import Path
from typing import Any, Mapping

from dev import generate_init_file, generate_stub

SCRIPT_DIR = Path(__file__).resolve().parent


def add_settings(
    s: Mapping[str, Any] | None = None, stubs: bool = True, release: bool = False
) -> Mapping[str, Any]:
    args = []
    if stubs:
        args.append("--generate-stubs")
    if release:
        args.append("--release")
    base = dict(s) if s else {}
    return base | {"maturin.build-args": args}


def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    return maturin.build_wheel(
        wheel_directory,
        # stubs should be generated before build_wheel.
        add_settings(config_settings, release=False, stubs=False),
        metadata_directory,
    )


def build_sdist(sdist_directory, config_settings=None):
    # generate stubs here
    generate_stub(maturin._get_env())
    generate_init_file(maturin._get_env())
    return maturin.build_sdist(
        sdist_directory, add_settings(config_settings, release=True)
    )


def build_editable(wheel_directory, config_settings=None, metadata_directory=None):
    return maturin.build_editable(
        wheel_directory, add_settings(config_settings), metadata_directory
    )


def get_requires_for_build_wheel(config_settings=None):
    return maturin.get_requires_for_build_wheel(
        add_settings(add_settings(config_settings, release=True), release=True)
    )


def get_requires_for_build_editable(config_settings=None):
    return maturin.get_requires_for_build_editable(add_settings(config_settings))


def prepare_metadata_for_build_wheel(metadata_directory, config_settings=None):
    return maturin.prepare_metadata_for_build_wheel(
        metadata_directory, add_settings(config_settings, release=True)
    )


def prepare_metadata_for_build_editable(metadata_directory, config_settings=None):
    return maturin.prepare_metadata_for_build_editable(
        metadata_directory, add_settings(config_settings)
    )
