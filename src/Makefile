# The build system uses a single Makefile (no recursive make), and does not use
# autotools or any other complex stuff.
# For info on writing Makefiles, see
#    The GNU Make manual: https://www.gnu.org/software/make/manual/html_node/index.html
#    The Makefile used to compile git: https://git.kernel.org/pub/scm/git/git.git/plain/Makefile
#                                 and: https://git.kernel.org/pub/scm/git/git.git/plain/config.mak.uname

include build/os_detection.mak
include build/pre_requisites.mak


# Determine what the name of the final program should be. We need to append
# '.exe' when building on Windows.
PYTHON_SRC_DIR = python
RUST_SRC_DIR = rust
EXE_SUFFIX =
ifeq ($(uname_S),Windows)
	EXE_SUFFIX = .exe
endif

# This is just 'qork' or 'qork.exe' on Windows.
EXE = qork$(EXE_SUFFIX)

# This is qork in the folder to which pyinstaller will write the bundled qork progam.
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
	    EXE_SUFFIX is $(EXE_SUFFIX), EXE is $(EXE), and BUNDLED_QORK_EXE is $(BUNDLED_QORK_EXE)\n"


# This tells make that these targets are "not real", in the sense that there
# is no file associated with them.
.PHONY: dump clean


