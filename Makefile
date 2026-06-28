NAME = llt

ZIG = zig

BUILD_DIR = build

CFLAGS = -Wall -Wextra -O2 -fPIC

MODULES = core tools

SRCS := $(shell find $(MODULES) -type f -name "*.c" | sort)
HEADERS := $(shell find $(MODULES) -type f -name "*.h" | sort)

INCLUDES := \
  -Icore/include \
  -Itools/include

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
	LIB_EXT = dylib
	SHARED_FLAG = -dynamiclib
endif

ifeq ($(UNAME_S),Linux)
	LIB_EXT = so
	SHARED_FLAG = -shared
endif

ifeq ($(OS),Windows_NT)
	LIB_EXT = dll
	SHARED_FLAG = -shared
endif

TARGET = $(BUILD_DIR)/lib$(NAME).$(LIB_EXT)

build:
	@mkdir -p $(BUILD_DIR)
	$(ZIG) cc $(SHARED_FLAG) $(CFLAGS) $(INCLUDES) $(SRCS) -o $(TARGET)

linux:
	@mkdir -p $(BUILD_DIR)/linux
	$(ZIG) cc -shared -target x86_64-linux $(CFLAGS) $(INCLUDES) $(SRCS) -o $(BUILD_DIR)/linux/lib$(NAME).so

windows:
	@mkdir -p $(BUILD_DIR)/windows
	$(ZIG) cc -shared -target x86_64-windows $(CFLAGS) $(INCLUDES) $(SRCS) -o $(BUILD_DIR)/windows/lib$(NAME).dll

darwin-x64:
	@mkdir -p $(BUILD_DIR)/darwin/x64
	$(ZIG) cc -dynamiclib -target x86_64-macos $(CFLAGS) $(INCLUDES) $(SRCS) -o $(BUILD_DIR)/darwin/x64/lib$(NAME).dylib

test:
	$(ZIG) cc $(CFLAGS) $(INCLUDES) tests/*.c $(SRCS) -o $(BUILD_DIR)/test
	./$(BUILD_DIR)/test

gen-go: build
	@mkdir -p api/go
	@cp $(TARGET) api/go/lib$(NAME).$(LIB_EXT)
	cd api/go && go run gen/main.go

info:
	@echo "OS: $(UNAME_S)"
	@echo "Arch: $(UNAME_M)"
	@echo "Sources: $(SRCS)"
	@echo "Includes: $(INCLUDES)"
	@echo "Target: $(TARGET)"

clean:
	rm -rf $(BUILD_DIR)

.PHONY: build linux windows darwin-x64 test gen-go info clean