.SUFFIXES: .txt2 .out

.txt2.out:
	@echo "Converting copied.txt to copied.out" 
	@cp copied.txt copied.out


copied.out: copied.txt2

