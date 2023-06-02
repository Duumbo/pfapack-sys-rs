OBJECTS= skpfa.o skpf10.o skbpfa.o skbpf10.o sktrf.o sktrd.o skbtrd.o

CC = gcc
CFLAGS = -O3
C_DIR = $(OUT_DIR)/c_interface

all: $(OBJECTS)
	$(AR) rvu $(C_DIR)/libcpfapack.a $(C_DIR)/$(OBJECTS)

clean:
	rm -f *.o
	rm -f libcpfapack.a

.SUFFIXES: .o .c

.c.o:
	$(CC) $(CFLAGS) -c $< -o $(C_DIR)/$@

