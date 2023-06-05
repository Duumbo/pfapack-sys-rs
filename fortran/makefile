FORT = gfortran
F_DIR = $(OUT_DIR)/fortran
FFLAGS = -O3 -fimplicit-none -moddir=$(F_DIR)/
OBJECTS= $(F_DIR)/slasktrd.o $(F_DIR)/slasktrf.o $(F_DIR)/sskbpf10.o $(F_DIR)/sskbpfa.o $(F_DIR)/sskbtrd.o \
	 $(F_DIR)/sskmv.o $(F_DIR)/sskpf10.o $(F_DIR)/sskpfa.o $(F_DIR)/sskr2.o $(F_DIR)/sskr2k.o $(F_DIR)/ssktd2.o \
         $(F_DIR)/ssktf2.o $(F_DIR)/ssktrd.o $(F_DIR)/ssktrf.o \
	 $(F_DIR)/dlasktrd.o $(F_DIR)/dlasktrf.o $(F_DIR)/dskbpf10.o $(F_DIR)/dskbpfa.o $(F_DIR)/dskbtrd.o \
	 $(F_DIR)/dskmv.o $(F_DIR)/dskpf10.o $(F_DIR)/dskpfa.o $(F_DIR)/dskr2.o $(F_DIR)/dskr2k.o $(F_DIR)/dsktd2.o \
	 $(F_DIR)/dsktf2.o $(F_DIR)/dsktrd.o $(F_DIR)/dsktrf.o \
	 $(F_DIR)/clasktrd.o $(F_DIR)/clasktrf.o $(F_DIR)/cskbpf10.o $(F_DIR)/cskbpfa.o $(F_DIR)/cskbtrd.o \
	 $(F_DIR)/cskmv.o $(F_DIR)/cskpf10.o $(F_DIR)/cskpfa.o $(F_DIR)/cskr2.o $(F_DIR)/cskr2k.o $(F_DIR)/csktd2.o \
	 $(F_DIR)/csktf2.o $(F_DIR)/csktrd.o $(F_DIR)/csktrf.o \
	 $(F_DIR)/zlasktrd.o $(F_DIR)/zlasktrf.o $(F_DIR)/zskbpf10.o $(F_DIR)/zskbpfa.o $(F_DIR)/zskbtrd.o \
	 $(F_DIR)/zskmv.o $(F_DIR)/zskpf10.o $(F_DIR)/zskpfa.o $(F_DIR)/zskr2.o $(F_DIR)/zskr2k.o $(F_DIR)/zsktd2.o \
	 $(F_DIR)/zsktf2.o $(F_DIR)/zsktrd.o $(F_DIR)/zsktrf.o $(F_DIR)/mul10.o\
	 $(F_DIR)/precision.o $(F_DIR)/f77_interface.o $(F_DIR)/f95_interface.o $(F_DIR)/message.o \
	 $(F_DIR)/skpfa.o $(F_DIR)/skpf10.o $(F_DIR)/skbpfa.o $(F_DIR)/skbpf10.o $(F_DIR)/sktrd.o $(F_DIR)/sktd2.o \
	 $(F_DIR)/sktrf.o $(F_DIR)/sktf2.o $(F_DIR)/skbtrd.o

OBJECTS_R= slasktrd.o slasktrf.o sskbpf10.o sskbpfa.o sskbtrd.o \
	 sskmv.o sskpf10.o sskpfa.o sskr2.o sskr2k.o ssktd2.o \
         ssktf2.o ssktrd.o ssktrf.o \
	 dlasktrd.o dlasktrf.o dskbpf10.o dskbpfa.o dskbtrd.o \
	 dskmv.o dskpf10.o dskpfa.o dskr2.o dskr2k.o dsktd2.o \
	 dsktf2.o dsktrd.o dsktrf.o \
	 clasktrd.o clasktrf.o cskbpf10.o cskbpfa.o cskbtrd.o \
	 cskmv.o cskpf10.o cskpfa.o cskr2.o cskr2k.o csktd2.o \
	 csktf2.o csktrd.o csktrf.o \
	 zlasktrd.o zlasktrf.o zskbpf10.o zskbpfa.o zskbtrd.o \
	 zskmv.o zskpf10.o zskpfa.o zskr2.o zskr2k.o zsktd2.o \
	 zsktf2.o zsktrd.o zsktrf.o mul10.o\
	 precision.o f77_interface.o f95_interface.o message.o \
	 skpfa.o skpf10.o skbpfa.o skbpf10.o sktrd.o sktd2.o \
	 sktrf.o sktf2.o skbtrd.o

all: $(OBJECTS_R)
	$(AR) rvu $(F_DIR)/libpfapack.a $(OBJECTS)

clean:
	rm -f *.o
	rm -f *.mod
	rm -f libpfapack.a

.SUFFIXES: .o .f .f90

.f90.o:
	$(FORT) $(FFLAGS) -c $< -o $(F_DIR)/$@

.f.o:
	$(FORT) $(FFLAGS) -c $< -o $(F_DIR)/$@
