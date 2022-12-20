.PHONY: all clean

CTARGETS := $(basename $(wildcard *.c))
CXXTARGETS := $(basename $(wildcard *.cpp))
RUSTTARGETS := $(basename $(wildcard *.rs))
GOTARGETS := $(basename $(wildcard *.go))
JAVATARGETS := $(basename $(wildcard *.java))
ASMTARGETS := $(basename $(wildcard *.asm))

all : $(CTARGETS) $(CXXTARGETS) $(RUSTTARGETS) $(GOTARGETS) $(JAVATARGETS) $(ASMTARGETS)

clean :
	@rm $(CTARGETS) $(CXXTARGETS) $(RUSTTARGETS) $(GOTARGETS) $(ASMTARGETS) *.class *.o 2>/dev/null || true

CFLAGS := -std=gnu17 -g
CXXFLAGS := -std=c++17 -g

% :: %.c
	$(CC) $(CFLAGS) $(CPPFLAGS) $< -o $@

% :: %.cpp
	$(CXX) $(CXXFLAGS) $< -o $@

% :: %.rs
	rustc --edition 2021 -O $<

% :: %.go
	go build $<

% :: %.java
	javac $<

% :: %.asm
	nasm -felf64 $< -o $@.o && ld -o $@ $@.o
