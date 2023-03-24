package main

import "fmt"

/*
Interestingly, I needed to define the function types - even if need to restate the signature
when creating the closures. 

This is because I didn't found a way to cast the "object" function calls in main
to the signature. See closure_point.go on how to cast directly 
Anyway, this looks better
*/

type addfnType func() 
type getfnType func() int

func closure_object() (addfnType, getfnType) {
	x := 1

	var addfn addfnType = func() {
		x += 1
	}

	var getfn getfnType = func() int {
		return x
	}

	return addfn, getfn
}

func main() {
	add1, get := closure_object()
	add1()
	add1()
	fmt.Println(get())
}
