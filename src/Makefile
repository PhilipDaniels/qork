# The build system uses a single Makefile (no recursive make), and does not use
# autotools or any other complex stuff.
# For info on writing Makefiles, see
#    The GNU Make manual: https://www.gnu.org/software/make/manual/html_node/index.html
#    The Makefile used to compile git: https://git.kernel.org/pub/scm/git/git.git/plain/Makefile
#                                 and: https://git.kernel.org/pub/scm/git/git.git/plain/config.mak.uname

# The default target of this Makefile is
all::

# Valid values for MODE are 'debug' and 'release', and VERBOSE can be 0 or 1.
# 1 is chattier. Example of setting from the command line:
#     $ make MODE=release VERBOSE=1
MODE=debug
VERBOSE=0

include build/os_detection.mak
include build/pre_requisites.mak
include build/names_and_paths.mak

CARGO_FLAGS =
ifeq ($(MODE),release)
	CARGO_FLAGS += --release
endif
ifeq ($(VERBOSE),1)
	CARGO_FLAGS += -v
endif

all:: dump $(PYTHON_QORK_EXE)

$(PYTHON_QORK_EXE): $(RUST_LIBQORK) $(PYTHON_SRC_DIR)/$(QORK).py
	cd $(PYTHON_SRC_DIR) && $(PYINSTALLER) --clean -y --log-level INFO $(QORK).py

rust_libs: $(RUST_LIBQORK)

$(RUST_LIBQORK): $(RUST_SRC_DIR)/$(QORK)/src/*.rs
	cd $(RUST_SRC_DIR)/$(QORK) && $(CARGO) build $(CARGO_FLAGS)



clean:
	$(RM) -rf $(PYTHON_SRC_DIR)/build $(PYTHON_SRC_DIR)/dist \
		$(PYTHON_SRC_DIR)/__pycache__ $(PYTHON_SRC_DIR)/$(QORK).spec \
		$(PYTHON_QORK_EXE) $(RUST_SRC_DIR)/$(QORK)/target

.PHONY: clean
