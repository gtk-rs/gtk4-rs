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


def def_check_submodule(submodule_path):
    if len(listdir(submodule_path)) != 0:
        return NOTHING_TO_BE_DONE
    print('=> Initializing gir submodule...')
    if not run_command(['git', 'submodule', 'update', '--init']):
        return FAILURE
    print('<= Done!')

    question = 'Do you want to update gir submodule? [y/N] '
    if sys.version_info[0] < 3:
        line = raw_input(question)
    else:
        line = input(question)
    line = line.strip()
    if line.lower() == 'y':
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
    elif updated_submodule == NEED_UPDATE or not isfile('./gir/target/release/gir'):
        print('=> Building gir...')
        if update_workspace() is True:
            print('<= Done!')
        else:
            print('<= Failed...')
            return False
    return True


def regen_crates(path, level=0):
    for entry in listdir(path):
        entry_file = join(path, entry)
        if isdir(entry_file):
            if level < 2 and not regen_crates(entry_file, level + 1):
                return False
        elif entry == 'Gir.toml':
            print('==> Regenerating "{}"...'.format(entry_file))

            args = ['./gir/target/release/gir', '-c', entry_file, '-o', path, '-d', 'gir-files']
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
                line = input('Do you want to continue? [y/N] ').strip().lower()
                if line != 'y':
                    return False
            print('<== Done!')
    return True


def main():
    if def_check_submodule("gir-files") == FAILURE:
        return 1
    if not build_gir_if_needed(def_check_submodule("gir")):
        return 1

    print('=> Regenerating crates...')
    if not regen_crates("."):
        return 1
    if not run_command(['cargo', 'fmt']):
        return 1
    print('<= Done!')
    print("Don't forget to check if everything has been correctly generated!")
    return 0



if __name__ == "__main__":
    sys.exit(main())