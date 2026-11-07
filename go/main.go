package main

import "fmt"

//go:noinline
func sumToN(n int32) int32 {
    var sum int32 = 0
    for i := int32(1); i <= n; i++ {
        sum += i
    }
    return sum
}

func main() {
    n := int32(10)
    fmt.Println(sumToN(n))
}