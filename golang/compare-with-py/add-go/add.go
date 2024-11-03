package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func main() {
	fmt.Println("Golang")
	limitStr := os.Getenv("UPPER_LIMIT")
	if limitStr == "" {
		panic("no limit")
	}
	limit, err := strconv.Atoi(limitStr)
	if err != nil {
		panic(err)
	}
	startTime := time.Now()
	data := []uint64{} //no preallocation to make it harder for go
	for i := 0; i < limit; i++ {
		data = append(data, uint64(i))
	}
	var res uint64 = 0
	for _, d := range data {
		res += d
	}
	endTime := time.Now()
	fmt.Println("Result: ", res)
	fmt.Println("Execution time:", endTime.Sub(startTime))
}
