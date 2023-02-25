CC=gcc
TARGET=vnc
 
build:
	$(CC) main.c -o $(TARGET)
 
clean:
	rm $(TARGET)