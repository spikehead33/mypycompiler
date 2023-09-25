import something

def f() -> Int:
    return 123

def g(a: Float, b: Float) -> Float:
    return a + b

def main():
    a: Int = 10
    b: Int = 20
    print(a + b)
    
    f()
    g(1.0, 2.0)
