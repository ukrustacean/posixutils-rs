.SUFFIXES: .txt .out  

.txt.ot:  
	@echo "Converting copied.txt to copied.out" 
	@cp copied.txt copied.out


copied.ot: copied.txt

