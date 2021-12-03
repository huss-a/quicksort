all: src/QuickSort.c
		mkdir -p bin
		gcc -o bin/qs src/QuickSort.c

dox: src/QuickSort.h src/QuickSort.c
		doxygen Doxyfile

clean:
		rm -rf bin