all:
	make -C c_interface
	make -C fortran

clean:
	make -C c_interface clean
	make -C fortran clean
