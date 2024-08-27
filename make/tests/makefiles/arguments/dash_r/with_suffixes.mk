.SUFFIXES: .txt .out  

.txt.out:  
	@echo "Converting suff.txt to suff.out" 
	@cp suff.txt suff.out


suff.out: suff.txt


