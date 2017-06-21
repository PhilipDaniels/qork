
# The name of the thing we are trying to build.
QORK = qork

# Subdirectories containing the main source code.
PYTHON_SRC_DIR = python
RUST_SRC_DIR = rust

EXE_SUFFIX =
ifeq ($(uname_S),Windows)
	EXE_SUFFIX = .exe
endif

DYN_LIB_SUFFIX = .so
ifeq ($(uname_S),Windows)
	DYN_LIB_SUFFIX = .dll
endif
ifeq ($(uname_S),Darwin)
	DYN_LIB_SUFFIX = .dylib
endif


# Hence the name of the final exe produced: just 'qork' or 'qork.exe' on Windows.
QORK_EXE = $(QORK)$(EXE_SUFFIX)
# This is qork in the folder to which pyinstaller will write the final Python qork progam.
PYTHON_QORK_EXE = $(PYTHON_SRC_DIR)/dist/$(QORK)/$(QORK_EXE)

# The various rust libs we need to build.
RUST_LIBQORK = $(RUST_SRC_DIR)/$(QORK)/target/$(MODE)/lib$(QORK)$(DYN_LIB_SUFFIX)

# Dump all variables. For several alternative techniques, see
# https://stackoverflow.com/questions/16467718/how-to-print-out-a-variable-in-makefile
dump:
	@echo -n "Dumping variables.\n\
	    uname_S = $(uname_S), uname_M = $(uname_M), uname_O = $(uname_O) and uname_P = $(uname_P)\n\
	    PYTHON_SRC_DIR = $(PYTHON_SRC_DIR) and RUST_SRC_DIR = $(RUST_SRC_DIR)\n\
	    DYN_LIB_SUFFIX = $(DYN_LIB_SUFFIX)\n\
	    RUST_LIBQORK = $(RUST_LIBQORK)\n\
	    QORK_EXE = $(QORK_EXE)\n\
	    PYTHON_QORK_EXE = $(PYTHON_QORK_EXE)  (this is the final program)\n"

.PHONY: dump
