def factorial(n):
    if n == 0:
        return 1 #### Why has to be ONE?
    return n * factorial(n - 1)
    
print(factorial(10))

