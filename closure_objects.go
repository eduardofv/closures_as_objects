package main

import "fmt"

func closure_point() map[string] interface{} {
	px := 0
	py := 0

	set := func(x, y int) {
		px = x
		py = y
	}

	get := func() (int, int) {
		return px, py
	}

	move := func() {
		px += 1
		py += 1
	}

	functions := make(map[string] interface{})

	functions["set"] = set
	functions["get"] = get
	functions["move"] = move

	return functions
}

func build_closure_point() (func() map[string] interface{}) {
	return closure_point
}

func main() {
	my_obj := closure_point()

	my_obj["set"].(func(int, int))(1, 2)
	fmt.Print("my_obj set at: "); 
	fmt.Println(my_obj["get"].(func() (int, int))())

	my_obj["move"].(func())()
	fmt.Print("my_obj after move: ")
	fmt.Println(my_obj["get"].(func() (int, int))())


	new_obj_container := build_closure_point()
	new_obj := new_obj_container()
	fmt.Print("new_obj created at: ")
	fmt.Println(new_obj["get"].(func() (int, int))())

	new_obj["move"].(func())()
	fmt.Print("new_obj after move: ")
	fmt.Println(new_obj["get"].(func() (int, int))())


	fmt.Print("my_obj still at: ")
	fmt.Println(my_obj["get"].(func() (int, int))())
}
