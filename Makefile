CC := gcc
CFLAGS := -g -Wall -Wextra 

all: rust_test

rust_test: rust_test
		$(CC) rust_test.c target/debug/librust_test.a -o rust_test
