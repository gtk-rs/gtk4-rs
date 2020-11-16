#!/usr/bin/env python3

from os import listdir
from os.path import isfile, isdir, join
import subprocess
import sys


NOTHING_TO_BE_DONE = 0
NEED_UPDATE = 1
FAILURE = 2


def run_command(command, folder=None):
    if folder is None:
        folder = "."
    child = subprocess.Popen(command, cwd=folder)
    child.communicate()
    if child.returncode != 0:
        print("Command `{}` failed with return code `{}`...".format(command, child.returncode))
        return False
    return True


def update_workspace():
    try:
        return run_command(['cargo', 'build', '--release'], 'gir')
    except:
        return False


def ask_yes_no_question(question, conf):
    question = '{} [y/N] '.format(question)
    if conf["yes"] is True:
        print(question)
        return True
    if sys.version_info[0] < 3:
        line = raw_input(question)
    else:
        line = input(question)
    return line.strip().lower() == 'y'


def def_check_submodule(submodule_path, conf):
    if len(listdir(submodule_path)) != 0:
        return NOTHING_TO_BE_DONE
    print('=> Initializing gir submodule...')
    if not run_command(['git', 'submodule', 'update', '--init']):
        return FAILURE
    print('<= Done!')

    if ask_yes_no_question('Do you want to update gir submodule?', conf):
        print('=> Updating gir submodule...')
        if not run_command(['git', 'reset', '--hard', 'HEAD'], 'gir'):
            return FAILURE
        if not run_command(['git', 'pull', '-f', 'origin', 'master'], 'gir'):
            return FAILURE
        print('<= Done!')
        return NEED_UPDATE
    return NOTHING_TO_BE_DONE


def build_gir_if_needed(updated_submodule):
    if updated_submodule == FAILURE:
        return False
    print('=> Building gir...')
    if update_workspace() is True:
        print('<= Done!')
    else:
        print('<= Failed...')
        return False
    return True


def regen_crates(path, conf, level=0):
    for entry in listdir(path):
        entry_file = join(path, entry)
        if isdir(entry_file):
            if level < 2 and not regen_crates(entry_file, conf, level + 1):
                return False
        elif entry == 'Gir.toml':
            print('==> Regenerating "{}"...'.format(entry_file))

            args = [conf["gir_path"], '-c', entry_file, '-o', path, '-d', conf["gir_files"]]
            if level > 1:
                args.append('-m')
                args.append('sys')
            error = False
            try:
                error = run_command(args) is False
            except Exception as err:
                print('The following error occurred: {}'.format(err))
                error = True
            if error is True:
                if not ask_yes_no_question('Do you want to continue?', conf):
                    return False
            print('<== Done!')
    return True


def print_help():
    print("generator.py            Helper to regenerate gtk-rs crates using gir.")
    print("")
    print("[OPTIONS]")
    print("  -h | --help           Display this message")
    print("  --gir-path [PATH]     Sets the path of the gir executable to run")
    print("                        (`./gir/target/release/gir` by default)")
    print("  --gir-files [PATH]    Sets the path of the gir-files folder")
    print("                        (`gir-files` by default)")
    print("  --yes                 Always answer `yes` to any question asked by the script")
    print("  --no-fmt              If set, this script won't run `cargo fmt`")


def parse_args(args):
    conf = {
        "gir_path": None,
        "gir_files": None,
        "yes": False,
        "run_fmt": True,
    }
    i = 0

    while i < len(args):
        arg = args[i]
        if arg == "-h" or arg == "--help":
            print_help()
            return None
        elif arg == "--gir-path":
            i += 1
            if i >= len(args):
                print("Expected argument after `--gir-path` option...")
                return None
            if not isfile(args[i]):
                print("`{}` file doesn't exist. Aborting...".format(args[i]))
                return None
            conf["gir_path"] = args[i]
        elif arg == "--gir-files":
            i += 1
            if i >= len(args):
                print("Expected argument after `--gir-files` option...")
                return None
            if not isdir(args[i]):
                print("`{}` folder doesn't exist. Aborting...".format(args[i]))
                return None
            conf["gir_files"] = args[i]
        elif arg == "--yes":
            conf["yes"] = True
        elif arg == "--no-fmt":
            conf["run_fmt"] = False
        else:
            print("Unknown argument `{}`.".format(arg))
            return None
        i += 1
    return conf


def main():
    gir_path = None

    conf = parse_args(sys.argv[1:])
    if conf is None:
        return 1

    if conf["gir_files"] is None:
        if def_check_submodule("gir-files", conf) == FAILURE:
            return 1
        conf["gir_files"] = "gir-files"
    if conf["gir_path"] is None:
        if not build_gir_if_needed(def_check_submodule("gir", conf)):
            return 1
        conf["gir_path"] = "./gir/target/release/gir"

    print('=> Regenerating crates...')
    if not regen_crates(".", conf):
        return 1
    if conf["run_fmt"] is True and not run_command(['cargo', 'fmt']):
        return 1
    print('<= Done!')
    print("Don't forget to check if everything has been correctly generated!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
