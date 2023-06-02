CC = gcc
CFLAGS = -O3
C_DIR = $(OUT_DIR)/c_interface
OBJECTS= $(C_DIR)/skpfa.o $(C_DIR)/skpf10.o $(C_DIR)/skbpfa.o $(C_DIR)/skbpf10.o $(C_DIR)/sktrf.o $(C_DIR)/sktrd.o $(C_DIR)/skbtrd.o

all: $(OBJECTS)
	$(AR) rvu $(C_DIR)/libcpfapack.a $(OBJECTS)

clean:
	rm -f *.o
	rm -f libcpfapack.a

.SUFFIXES: .o .c

.c.o:
	$(CC) $(CFLAGS) -c $< -o $(C_DIR)/$@

