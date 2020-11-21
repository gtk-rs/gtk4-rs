#!/usr/bin/env python3

from os import listdir
from os.path import isfile, isdir, join
import argparse
import subprocess
import sys


NOTHING_TO_BE_DONE = 0
NEED_UPDATE = 1
FAILURE = 2

DEFAULT_GIR_DIRECTORY = 'gir-files'
DEFAULT_GIR_PATH = './gir/target/release/gir'


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
    if conf.yes:
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
    if update_workspace():
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
        elif entry.startswith("Gir") and entry.endswith(".toml"):
            print('==> Regenerating "{}"...'.format(entry_file))

            args = [conf.gir_path, '-c', entry_file, '-o', path, '-d', conf.gir_directory]
            if level > 1:
                args.append('-m')
                args.append('sys')
            error = False
            try:
                error = not run_command(args)
            except Exception as err:
                print('The following error occurred: {}'.format(err))
                error = True
            if error:
                if not ask_yes_no_question('Do you want to continue?', conf):
                    return False
            print('<== Done!')
    return True


def parse_args(args):
    parser = argparse.ArgumentParser(description='Helper to regenerate gtk-rs crates using gir.',
                                     formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    parser.add_argument('path', nargs="*", default='.',
                        help='Paths in which to look for Gir.toml files')
    parser.add_argument('--gir-directory', default=DEFAULT_GIR_DIRECTORY,
                        help='Path of the gir-files folder')
    parser.add_argument('--gir-path', default=DEFAULT_GIR_PATH,
                        help='Path of the gir executable to run')
    parser.add_argument('--yes', action='store_true',
                        help=' Always answer `yes` to any question asked by the script')
    parser.add_argument('--no-fmt', action='store_true',
                        help='If set, this script will not run `cargo fmt`')

    return parser.parse_args()


def main():
    gir_path = None

    conf = parse_args(sys.argv[1:])

    if conf.gir_directory == DEFAULT_GIR_DIRECTORY:
        if def_check_submodule("gir-files", conf) == FAILURE:
            return 1
    elif not isdir(conf.gir_directory):
        print("`{}` dir doesn't exist. Aborting...".format(path))
        return 1

    if conf.gir_path == DEFAULT_GIR_PATH:
        if not build_gir_if_needed(def_check_submodule("gir", conf)):
            return 1
    elif not isfile(conf.gir_path):
        print("`{}` file doesn't exist. Aborting...".format(path))
        return 1

    print('=> Regenerating crates...')
    for path in conf.path:
        print('=> Looking in path `{}`'.format(path))
        if not regen_crates(path, conf):
            return 1
    if not conf.no_fmt and not run_command(['cargo', 'fmt']):
        return 1
    print('<= Done!')
    print("Don't forget to check if everything has been correctly generated!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
