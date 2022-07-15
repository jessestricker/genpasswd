import logging
import subprocess


def capture_command(args: list[str]) -> str:
    output = _exec_command(args, text=True, capture_output=True)
    if output is None:
        return ""
    return output


def run_command(args: list[str]):
    _exec_command(args)


def _exec_command(args: list[str], **run_args) -> str | None:
    logging.debug("executing %s", args)
    result = subprocess.run(args, check=True, **run_args)
    return result.stdout
