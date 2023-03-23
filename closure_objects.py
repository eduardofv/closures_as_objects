def closure():
    x = 1
    
    def add():
        nonlocal x
        x += 1

    def get():
        nonlocal x
        return x

    return add, get


def main():
    add1, get = closure()
    add1()
    add1()
    print(get())

main()
