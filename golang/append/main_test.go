package main_test

import (
	"testing"
	"time"

	"golang.org/x/exp/rand"
)

var numbs []int

func BenchmarkPreallocatedInsert(b *testing.B) {
	for n := 0; n < b.N; n++ {
		slice := make([]int, 1000)
		for i := 0; i < 1000; i++ {
			slice[i] = i
		}
		numbs = slice
	}
}

func BenchmarkAppend(b *testing.B) {
	for n := 0; n < b.N; n++ {
		slice := make([]int, 0, 1000)
		for i := 0; i < 1000; i++ {
			// a := len(slice)
			s := append(slice, i)
			// b := len(slice)
			slice = s
			// if i%10 == 0 {
			// 	fmt.Println(fmt.Sprint(a) + " " + fmt.Sprint(b) + " " + fmt.Sprint(len(slice)))
			// }

		}
		numbs = slice
	}
}

var res []bool

func randomInt64() int64 {
	rand.Seed(uint64(time.Now().UnixNano()))
	return rand.Int63() // Int63 returns a non-negative pseudo-random 63-bit integer as an int64
}

func BenchmarkTimestampComp(b *testing.B) {
	in1 := make([]int64, 0, 1_000_000)
	in2 := make([]int64, 0, 1_000_000)
	out := make([]bool, 1_000_000)
	for i := 0; i < 1_000_000; i++ {
		in1 = append(in1, randomInt64())
		in2 = append(in2, randomInt64())
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for i := 0; i < 1_000_000; i++ {
			out[i] = in1[i] >= in2[i]
		}
	}
	res = out
}
