def infinite_recursion(n):
    if n <= 20:  # Base case to stop recursion is WRONG
        return 0
    print("This is a recursive call!")
    print(n)
    return n + infinite_recursion(n + 1)  

if __name__ == "__main__":
    infinite_recursion(10)