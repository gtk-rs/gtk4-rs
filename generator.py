#!/usr/bin/env python3

from pathlib import Path
import argparse
import subprocess
import sys


NOTHING_TO_BE_DONE = 0
NEED_UPDATE = 1
FAILURE = 2

DEFAULT_GIR_FILES_DIRECTORY = Path("./gir-files")
DEFAULT_GIR_DIRECTORY = Path("./gir/")
DEFAULT_GIR_PATH = DEFAULT_GIR_DIRECTORY / "target/release/gir"


def run_command(command, folder=None):
    if folder is None:
        folder = "."
    ret = subprocess.run(command, cwd=folder)
    if ret.returncode != 0:
        print("Command `{}` failed with `{}`...".format(command, ret))
        return False
    return True


def update_workspace():
    return run_command(["cargo", "build", "--release"], "gir")


def ask_yes_no_question(question, conf):
    question = "{} [y/N] ".format(question)
    if conf.yes:
        print(question + "y")
        return True
    line = input(question)
    return line.strip().lower() == "y"


def def_check_submodule(submodule_path, conf):
    if any(submodule_path.iterdir()):
        return NOTHING_TO_BE_DONE
    print("=> Initializing {} submodule...".format(submodule_path))
    if not run_command(["git", "submodule", "update", "--init", submodule_path]):
        return FAILURE
    print("<= Done!")

    if ask_yes_no_question(
        "Do you want to update {} submodule?".format(submodule_path), conf
    ):
        print("=> Updating submodule...")
        if not run_command(["git", "reset", "--hard", "HEAD"], submodule_path):
            return FAILURE
        if not run_command(["git", "pull", "-f", "origin", "master"], submodule_path):
            return FAILURE
        print("<= Done!")
        return NEED_UPDATE
    return NOTHING_TO_BE_DONE


def build_gir_if_needed(updated_submodule):
    if updated_submodule == FAILURE:
        return False
    print("=> Building gir...")
    if update_workspace():
        print("<= Done!")
    else:
        print("<= Failed...")
        return False
    return True


def regen_crates(path, conf):
    if path.is_dir():
        for entry in path.rglob("Gir*.toml"):
            if not regen_crates(entry, conf):
                return False
    elif path.match("Gir*.toml"):
        print('==> Regenerating "{}"...'.format(path))

        args = [conf.gir_path, "-c", path, "-o", path.parent] + [
            d for path in conf.gir_files_paths for d in ("-d", path)
        ]
        if path.parent.name.endswith("sys"):
            args.extend(["-m", "sys"])
        error = False
        try:
            error = not run_command(args)
        except Exception as err:
            print("The following error occurred: {}".format(err))
            error = True
        if error:
            if not ask_yes_no_question("Do you want to continue?", conf):
                return False
        print("<== Done!")
    else:
        print("==> {} is not a valid Gir*.toml file".format(path))
        return False
    return True


def valid_path(path):
    path = Path(path)
    if not path.exists():
        raise argparse.ArgumentTypeError("`{}` no such file or directory".format(path))
    return path


def directory_path(path):
    path = Path(path)
    if not path.is_dir():
        raise argparse.ArgumentTypeError("`{}` directory not found".format(path))
    return path


def file_path(path):
    path = Path(path)
    if not path.is_file():
        raise argparse.ArgumentTypeError("`{}` file not found".format(path))
    return path


def parse_args():
    parser = argparse.ArgumentParser(
        description="Helper to regenerate gtk-rs crates using gir.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "path",
        nargs="*",
        default=[Path(".")],
        type=valid_path,
        help="Paths in which to look for Gir.toml files",
    )
    parser.add_argument(
        "--gir-files-directories",
        nargs="+",  # If the option is used, we expect at least one folder!
        dest="gir_files_paths",
        default=[],
        type=directory_path,
        help="Path of the gir-files folder",
    )
    parser.add_argument(
        "--gir-path",
        default=DEFAULT_GIR_PATH,
        type=file_path,
        help="Path of the gir executable to run",
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help=" Always answer `yes` to any question asked by the script",
    )
    parser.add_argument(
        "--no-fmt",
        action="store_true",
        help="If set, this script will not run `cargo fmt`",
    )

    return parser.parse_args()


def main():
    conf = parse_args()

    if not conf.gir_files_paths:
        if def_check_submodule(DEFAULT_GIR_FILES_DIRECTORY, conf) == FAILURE:
            return 1

    if conf.gir_path == DEFAULT_GIR_PATH:
        if not build_gir_if_needed(def_check_submodule(DEFAULT_GIR_DIRECTORY, conf)):
            return 1

    print("=> Regenerating crates...")
    for path in conf.path:
        print("=> Looking in path `{}`".format(path))
        if not regen_crates(path, conf):
            return 1
    if not conf.no_fmt and not run_command(["cargo", "fmt"]):
        return 1
    print("<= Done!")
    print("Don't forget to check if everything has been correctly generated!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
