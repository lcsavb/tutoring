def hanoi(n, source, auxiliary, target):
    if n == 1:
        print('Move disk 1 from source', source, 'to target', target)
        return
    hanoi(n-1, source, target, auxiliary)
    print('Move disk', n, 'from source', source, 'to target', target)
    hanoi(n-1, auxiliary, source, target)
    print('Move disk', n, 'from source', source, 'to target', target)
    hanoi(n-1, source, auxiliary, target)

hanoi(3, 'A', 'B', 'C')
