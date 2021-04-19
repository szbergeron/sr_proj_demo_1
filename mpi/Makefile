CC=mpicc
LFLAGS=-lpthread -ldl
CFLAGS= -std=c99 -pedantic -Wall -ffloat-store $(LFLAGS) -lm

.PHONY: rust, spell-check, spell-check_debug

RTARGD=./target/debug/libmpi.a
RTARG=./target/release/libmpi.a

SRC=	./src/mpi.c

all: mpi mpi_debug

rust:
	cargo build
	cargo build --release

mpi: $(SRC)
	$(CC) $(CFLAGS) -DNDEBUG -O2 $^ -o mpi $(RTARG)


mpi_debug: $(SRC)
	$(CC) $(CFLAGS) -g $^ -o mpi_debug $(RTARGD)

clean:
	rm -f mpi mpi_debug
	rm -f *.o

