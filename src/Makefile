# The build system uses a single Makefile (no recursive make), and does not use
# autotools or any other complex stuff.
# For info on writing Makefiles, see
#    The GNU Make manual: https://www.gnu.org/software/make/manual/html_node/index.html
#    The Makefile used to compile git: https://git.kernel.org/pub/scm/git/git.git/plain/Makefile
#                                 and: https://git.kernel.org/pub/scm/git/git.git/plain/config.mak.uname

include build/os_detection.mak



# Define V = 1 for a more verbose build
V = 0

# Pre-requisites that we require to be installed in order to build. The Makefile
# will check these and print a nice error message if any is missing.
CP = cp
LS = ls
FIND = find
RM = rm
GIT = git
PYTHON = python3
PYINSTALLER = pyinstaller
PRE_REQUISITES = $(CP) $(LS) $(FIND) $(RM) $(GIT) $(PYTHON) $(PYINSTALLER)

$(info Checking that all required pre-requisites are installed on this computer...)
$(foreach exe,$(PRE_REQUISITES),\
		$(if $(shell which $(exe)),$(info Found $(exe)...),$(error No $(exe) in PATH. Please install it.)))
$(info Pre-requisite check completed.)

# Determine what the name of the final program should be. We need to append
# '.exe' when building on Windows.
PYTHON_SRC_DIR = python
RUST_SRC_DIR = rust
EXE_SUFFIX =
ifeq ($(uname_S),Windows)
	EXE_SUFFIX = .exe
endif
EXE = qork$(EXE_SUFFIX)
BUNDLED_QORK_EXE = $(PYTHON_SRC_DIR)/dist/qork/$(EXE)

# The default target of this Makefile is
all: dump $(BUNDLED_QORK_EXE)


$(BUNDLED_QORK_EXE): $(PYTHON_SRC_DIR)/$(EXE).py
	cd $(PYTHON_SRC_DIR) && \
		$(PYINSTALLER) --clean -y --log-level INFO $(EXE).py

clean:
	$(RM) -rf $(PYTHON_SRC_DIR)/build $(PYTHON_SRC_DIR)/dist \
		$(PYTHON_SRC_DIR)/__pycache__ $(PYTHON_SRC_DIR)/qork.spec \
		$(BUNDLED_QORK_EXE)


# Dump all variables. For several alternative techniques, see
# https://stackoverflow.com/questions/16467718/how-to-print-out-a-variable-in-makefile
dump:
	@echo -n "Dumping variables.\n\
	    uname_S is $(uname_S), uname_M is $(uname_M), uname_O is $(uname_O) and uname_P is $(uname_P)\n\
	    PYTHON_SRC_DIR is $(PYTHON_SRC_DIR) and RUST_SRC_DIR is $(RUST_SRC_DIR)\n\
	    EXE_SUFFIX is $(EXE_SUFFIX), EXE is $(EXE), and PROGRAM is $(PROGRAM)\n"


# This tells make that these targets are "not real", in the sense that there
# is no file associated with them.
.PHONY: dump clean


