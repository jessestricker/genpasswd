import json
from collections.abc import Mapping
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import common


@dataclass
class Target:
    name: str
    kind: list[str]


def _target_from_mapping(values: Mapping[str, Any]) -> Target:
    return Target(
        name=values["name"],
        kind=values["kind"],
    )


@dataclass
class Package:
    name: str
    version: str
    manifest_path: Path
    targets: list[Target]


def _package_from_mapping(values: Mapping[str, Any]) -> Package:
    return Package(
        name=values["name"],
        version=values["version"],
        manifest_path=Path(values["manifest_path"]),
        targets=[_target_from_mapping(x) for x in values["targets"]],
    )


@dataclass
class Metadata:
    version: int
    packages: list[Package]


def _metadata_from_mapping(values: Mapping[str, Any]) -> Metadata:
    return Metadata(
        version=values["version"],
        packages=[_package_from_mapping(x) for x in values["packages"]],
    )


def load_metadata(format_version: int = 1) -> Metadata:
    metadata_json = fetch_metadata_json(format_version)
    metadata = parse_metadata_from_json(metadata_json)
    return metadata


def parse_metadata_from_json(metadata_json: str) -> Metadata:
    metadata_dict = json.loads(metadata_json)
    metadata = _metadata_from_mapping(metadata_dict)
    return metadata


def fetch_metadata_json(format_version: int = 1) -> str:
    cmd_args = ["cargo", "metadata"]
    cmd_args += ["--no-deps"]  # skip metadata about dependencies
    cmd_args += ["--locked"]  # require a `Cargo.lock` file
    cmd_args += ["--format-version", str(format_version)]
    return common.capture_command(cmd_args)


def get_default_target_triple() -> str:
    lines = common.capture_command(["rustc", "-vV"]).splitlines()
    prefix = "host: "
    for line in lines:
        if line.startswith(prefix):
            return line[len(prefix) :]
    raise RuntimeError("`rustc -vV` does not output the 'host' field")
