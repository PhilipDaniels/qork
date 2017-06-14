#!/usr/bin/env python3

import os
import shutil
import sys
import subprocess

# Main build script for qork.

# Function to check whether a command exists.
cmd_exists = lambda x: shutil.which(x) is not None

# Function which prints to stderr and then exits with a non-success exit code.
def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)
    sys.exit(1)


print("Beginning build.")
cmd_py_ins = "pyinstaller"
cmd_py_ins_log_level = "INFO"

pyinstaller_exists = cmd_exists(cmd_py_ins)
if not pyinstaller_exists:
    eprint("The command {} does not exist. Please install it (using pip3).".format(cmd_py_ins))

# pyinstaller cannot do truly clean out-of-tree builds, so we won't bother.
# Instead, we will change to the relevant folder.
python_src_dir = os.path.abspath("../src/python")
os.chdir(python_src_dir)
print("Changed to directory " + python_src_dir)

# Invoke pyinstaller on the main qork.py file. The resultant executable will be
# written to "src/python/dist/qork[.exe]".
p = subprocess.run([cmd_py_ins, "--clean", "-F", "--log-level", cmd_py_ins_log_level, "qork.py"])
if p.returncode != 0:
    eprint("{} returned exit status code of {}. Stopping the build.".format(cmd_py_ins, p.returncode))
else:
    print("{} completed successfully.".format(cmd_py_ins))

# Check that the executable was actually produced.
exe_name = "qork.exe" if os.name == "nt" else "qork"
exe_path = os.path.abspath(os.path.join("dist", exe_name))
exe_size = os.stat(exe_path).st_size
if exe_size > 0:
    print("The executable {} was generated, has size {:,} bytes.".format(exe_path, exe_size))
else:
    eprint("The executable {} was not produced. Stopping the build.".format(exe_path));



print("Build completed.")

