def greet(name):
    print("Hello, " + name)

greet("World")

def add(a, b):
    return a + b

result = add(3, 4)
print("3 + 4 =", result)

def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print("5! =", factorial(5))