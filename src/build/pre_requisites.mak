# Pre-requisites that we require to be installed in order to build. The Makefile
# will check these and print a nice error message if any is missing.

CP = cp
LS = ls
FIND = find
RM = rm
GIT = git
PYTHON = python3
PYINSTALLER = pyinstaller
CARGO = cargo
RUSTC = rustc
RUSTFMT = rustfmt

PRE_REQUISITES = $(CP) $(LS) $(FIND) $(RM) $(GIT) $(PYTHON) $(PYINSTALLER) \
    $(CARGO) $(RUSTC) $(RUSTFMT)

$(info Checking that all required pre-requisites are installed on this computer...)
$(foreach exe,$(PRE_REQUISITES),\
		$(if $(shell which $(exe)),,$(error The program '$(exe)' is not in the PATH. Please install it)))
$(info Pre-requisite check completed.)


