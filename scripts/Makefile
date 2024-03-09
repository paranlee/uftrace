srcdir = $(CURDIR)/..
incdir = $(srcdir)/include

CC  := gcc
CXX := g++
RUSTC := rustc
CFLAGS   := -fPIC -shared -g -I$(incdir)
CXXFLAGS := $(CFLAGS)
RUSTFLAGS := --crate-type cdylib

CC	= gcc
SRCS	= simple.c info.c count.c dump.c simple.cpp simple.rs
CSO     = $(SRCS:%.c=%_c.so)
CPPSO   = $(SRCS:%.cpp=%_cpp.so)
RUSTSO   = $(SRCS:%.rs=%_rs.so)

all: $(CSO) $(CPPSO) $(RUSTSO)

%_c.so: %.c
	$(CC) $(CFLAGS) -o $@ $<

%_cpp.so: %.cpp
	$(CC) $(CXXFLAGS) -o $@ $<

%_rs.so: %.rs
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

clean:
	rm –f *.so
