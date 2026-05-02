def fib(n):
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

print("Fibonacci:")
print("F(0) =", fib(0))
print("F(1) =", fib(1))
print("F(5) =", fib(5))
print("F(10) =", fib(10))

def sum_list(lst):
    total = 0
    for x in lst:
        total = total + x
    return total

nums = [1, 2, 3, 4, 5]
print("Sum:", sum_list(nums))

def max_of_two(a, b):
    if a > b:
        return a
    return b

print("Max:", max_of_two(10, 20))