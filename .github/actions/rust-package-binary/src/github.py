class Error(Exception):
    title: str

    def __init__(self, title: str, message: str):
        super().__init__(message)
        self.title = title


class LogGroup:
    _title: str

    def __init__(self, title: str):
        self._title = title

    def __enter__(self):
        _exec_workflow_command("group", self._title)

    def __exit__(self, exc_type, exc_value, traceback):
        _exec_workflow_command("endgroup")


def handle_error(error: Error):
    set_error_message(error.title, str(error))


def set_output_parameter(name: str, value: str):
    _exec_workflow_command("set-output", value, {"name": name})


def set_error_message(title: str, message: str):
    _exec_workflow_command("error", message, {"title": title})


def _exec_workflow_command(
    name: str, value: str | None = None, args: dict[str, str] | None = None
):
    command = f"::{name}"

    if args is not None and len(args) > 0:
        args_str = ",".join(f"{k}={v}" for (k, v) in args.items())
        command += f" {args_str}"

    if value is not None:
        command += f"::{value}"

    print(command, flush=True)
