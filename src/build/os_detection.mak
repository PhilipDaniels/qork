# Determine which OS we are building on. Based on the answer at
# https://stackoverflow.com/questions/714100/os-detecting-makefile
# and https://git.kernel.org/pub/scm/git/git.git/plain/config.mak.uname

# The kernel name, e.g. "Linux" or "Windows"
ifeq ($(OS),Windows_NT)
    uname_S := Windows
else
    uname_S := $(shell sh -c 'uname -s 2>/dev/null || echo not')
endif

# The machine hardware name, e.g. "x86_64".
uname_M := $(shell sh -c 'uname -m 2>/dev/null || echo not')

# The operating system, e.g. "GNU/Linux".
uname_O := $(shell sh -c 'uname -o 2>/dev/null || echo not')

# The processor type, e.g. "x86_64".
uname_P := $(shell sh -c 'uname -p 2>/dev/null || echo not')
