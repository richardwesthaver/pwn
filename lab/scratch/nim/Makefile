vpath %.exe bin/
vpath %.dll bin/
vpath %.nim src/

SRCS_BINS = $(notdir $(wildcard src/*_bin.nim))
SRCS_LIBS = $(notdir $(wildcard src/*_lib.nim))

TARGET ?= win64

ifeq ($(TARGET),win64)
  FLAGS = -d=debug -d=mingw --cpu=amd64 --embedsrc=on --hints=on
  BINS = $(patsubst %_bin.nim,%.exe,$(SRCS_BINS))
  DLLS = $(patsubst %_lib.nim,%.dll,$(SRCS_LIBS))
else ifeq ($(TARGET),win32)
  FLAGS = -d=debug -d=mingw --cpu=i386 --embedsrc=on --hints=on
  BINS = $(patsubst %_bin.nim,%.exe,$(SRCS_BINS))
  DLLS = $(patsubst %_lib.nim,%.dll,$(SRCS_LIBS))
else ifeq ($(TARGET),macos)
  FLAGS = --os:macosx
  BINS = $(patsubst %_bin.nim,%,$(SRCS_BINS))
  DLLS = $(patsubst %_lib.nim,%.so,$(SRCS_LIBS))
else ifeq ($(TARGET),linux)
  FLAGS = -d=debug --os:linux
  BINS = $(patsubst %_bin.nim,%,$(SRCS_BINS))
  DLLS = $(patsubst %_lib.nim,%.so,$(SRCS_LIBS))
endif

#OPT_FLAGS = -d=danger -d=mingw -d=strip --passc=-flto --passl=-flto --opt=size

.PHONY: clean
default: build
build: $(BINS) $(DLLS)
rebuild: clean build
clean:; rm -rf bin/*

% : %_bin.nim
	nim c $(FLAGS) --app=console --out=bin/$* $<

%.so : %_lib.nim
	nim c $(FLAGS) --app=lib --nomain --out=bin/$*.so $<

%.exe : %_bin.nim
	nim c $(FLAGS) --app=console --out=bin/$*_64.exe $<

%.dll: %_lib.nim
	nim c $(FLAGS) --app=lib --nomain --out=bin/$*_64.dll $<
