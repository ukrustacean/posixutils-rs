.SUFFIXES: .txt .ot  # Define the suffixes

.txt.ot:  
	@echo "Converting copied.txt to copied.out" 
	@cp copied.txt copied.ot


# Specify the dependency for copied.out
copied.ot: copied.txt

