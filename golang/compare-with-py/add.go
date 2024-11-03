package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("Golang")
	startTime := time.Now()
	data := []uint64{} //no preallocation to make it harder for go
	for i := 0; i < 1000; i++ {
		data = append(data, uint64(i))
	}
	var res uint64 = 0
	for _, d := range data {
		res += d * 2
	}
	endTime := time.Now()
	fmt.Println("Result: ", res)
	fmt.Println("Execution time:", endTime.Sub(startTime))
}
