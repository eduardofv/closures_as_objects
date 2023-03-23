/*

2023-03-21

how can I simulate objects using closures in go?

In Go, you can use closures to simulate objects by creating a closure that captures the state of an object and returns methods that operate on that state. Here's an example of how you can create an object-like structure using closures:
*/

package main

import "fmt"

type Counter struct {
    count int
}

func NewCounter() *Counter {
    c := &Counter{count: 0}

    // Return closure functions that can manipulate the state
    return &Counter{
        inc: func() {
            c.count++
        },
        dec: func() {
            c.count--
        },
        getCount: func() int {
            return c.count
        },
    }
}

func main() {
    c := NewCounter()

    c.inc()
    c.inc()
    c.dec()

    fmt.Println(c.getCount()) // Output: 1
}

/*
In this example, we define a Counter struct with an int field count, and then define a NewCounter function that returns a closure with methods that operate on the count field.

The NewCounter function creates a new instance of the Counter struct and returns a pointer to that instance. The closure that is returned captures the Counter instance and returns three closure methods: inc, dec, and getCount.

The inc method increments the count field of the Counter instance, the dec method decrements it, and the getCount method returns the current value of count.

In the main function, we create a new Counter instance using NewCounter, and then use its methods to manipulate its state and print its current value.

This pattern of using closures to simulate objects is a common idiom in Go and can be used to create simple, self-contained objects with encapsulated state and behavior.
*/
