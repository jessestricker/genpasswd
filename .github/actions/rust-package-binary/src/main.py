from __future__ import annotations

import argparse
import logging
import tempfile
import zipfile
from dataclasses import dataclass
from pathlib import Path

import cargo
import common
import github


def main():
    logging.basicConfig(
        level=logging.DEBUG,
        format="[{levelname:>7}] {message}",
        style="{",
    )

    # read inputs from CLI
    inputs = Inputs.parse_cli()
    logging.info("inputs: %s", inputs)

    # if not set, get default target
    if inputs.target is None:
        inputs.target = cargo.get_default_target_triple()
        logging.info("using default target: %s", inputs.target)

    # get metadata from Cargo
    metadata = cargo.load_metadata()
    logging.debug("metadata: %s", metadata)

    # if not set, get default package
    if inputs.package is None:
        if len(metadata.packages) != 1:
            raise ValueError(
                "The Cargo workspace contains multiple packages, "
                "but the input `package` was not set."
            )
        inputs.package = metadata.packages[0].name
        logging.info("using default package: %s", inputs.package)

    # find requested package
    package = select_package(inputs.package, metadata)
    logging.info("package: %s", package)

    # install binaries
    binaries_dir = install_package(package, inputs.target)
    logging.info("binaries directory: %s", binaries_dir)

    # package binaries in archive file
    archive_file = package_binaries(binaries_dir, package, inputs.target)
    logging.info("archive file: %s", archive_file)

    # write GitHub Workflow output parameters
    github.set_output_parameter("archive-file", str(archive_file))
    github.set_output_parameter("archive-name", str(archive_file.name))


@dataclass
class Inputs:
    package: str | None = None
    target: str | None = None

    @staticmethod
    def parse_cli() -> Inputs:
        parser = argparse.ArgumentParser()
        parser.add_argument("--package")
        parser.add_argument("--target")

        inputs = Inputs()
        parser.parse_args(namespace=inputs)
        return inputs


def select_package(package_name: str, metadata: cargo.Metadata) -> cargo.Package:
    package = next(
        (pkg for pkg in metadata.packages if pkg.name == package_name),
        None,
    )
    if package is None:
        raise ValueError(
            f"The Cargo workspace does not contain a package called '{package_name}'"
        )
    return package


def install_package(package: cargo.Package, target: str) -> Path:
    package_dir = package.manifest_path.parent
    install_dir = Path(tempfile.mkdtemp())

    cmd_args = ["cargo", "install"]
    cmd_args += ["--path", str(package_dir)]
    cmd_args += ["--root", str(install_dir)]
    cmd_args += ["--force", "--no-track"]
    cmd_args += ["--locked", "--all-features"]
    cmd_args += ["--target", target]
    cmd_args += ["--verbose"]

    with github.LogGroup("Installing package binaries"):
        common.run_command(cmd_args)

    binaries_dir = install_dir / "bin"
    return binaries_dir


def package_binaries(binaries_dir: Path, package: cargo.Package, target: str) -> Path:
    binary_files = list(binaries_dir.rglob("*"))

    archive_name = f"{package.name}-{package.version}-{target}.zip"
    archive_file = Path(tempfile.mkdtemp()) / archive_name

    with github.LogGroup("Built binary files"):
        with zipfile.ZipFile(
            archive_file, mode="x", compression=zipfile.ZIP_DEFLATED
        ) as archive:
            for binary_file in binary_files:
                archive_member_name = binary_file.relative_to(binaries_dir)
                archive.write(binary_file, archive_member_name)
                print(archive_member_name)

    return archive_file


if __name__ == "__main__":
    main()
