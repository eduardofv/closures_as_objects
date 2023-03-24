def closure_point():
    px = 0
    py = 0

    def setfn(x, y):
        nonlocal px, py
        px = x
        py = y

    def getfn():
        nonlocal px, py
        return (px, py)

    def move():
        nonlocal px, py
        px += 1
        py += 1

    return {'set': setfn, 'get': getfn, 'move': move}


def main():
    my_obj = closure_point()
    my_obj['set'](1, 2)
    print(f"my_obj set at: ", my_obj['get']())
    my_obj['move']()
    print(f"my_obj after move: ", my_obj['get']())

    new_obj = closure_point()
    print(f"new_obj created at: ", new_obj['get']())
    new_obj['move']()
    print(f"new_obj after move: ", new_obj['get']())

    print(f"my_obj still at: ", my_obj['get']())


main()
