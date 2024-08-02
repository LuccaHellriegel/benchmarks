package duplicates_test

import (
	"duplicates"
	"fmt"
	"testing"
	"time"

	"github.com/google/uuid"
	"golang.org/x/exp/rand"
)

const n = 100_000

var globalKeys []string
var globalData []*duplicates.Data
var globalDataSlices [][]*duplicates.Data
var globalMap map[string][]*duplicates.Data

// func Benchmark_NoMap(b *testing.B) {
// 	runNoMap(b, duplicates.DuplicatesNoMap)
// }

func Benchmark_NaiveIndices(b *testing.B) {
	run(b, duplicates.DuplicatesNaiveIndices)
}

func Benchmark_Naive(b *testing.B) {
	run(b, duplicates.DuplicatesNaive)
}

func Benchmark_Avoid(b *testing.B) {
	run(b, duplicates.DuplicatesAvoidFirstMap)
}

////////
// UTILS
////////

func run(b *testing.B, cb func(keys []string, data []*duplicates.Data) ([]string, []*duplicates.Data, map[string][]*duplicates.Data)) {
	run := func(dups int, amount int) {
		b.Run(fmt.Sprint(dups)+"dups_"+fmt.Sprint(amount)+"amount", func(b *testing.B) {
			keys, data := generateWithDups(n, dups, amount)
			b.ResetTimer()

			//execute
			var resKeys []string
			var resData []*duplicates.Data
			var resMap map[string][]*duplicates.Data
			for i := 0; i < b.N; i++ {
				resKeys, resData, resMap = cb(keys, data)
			}
			globalKeys = resKeys
			globalData = resData
			globalMap = resMap
		})
	}

	run(0, 0)

	for _, dups := range []int{1, 5, 10, 50, 100} {
		for _, amount := range []int{2, 10, 50} {
			run(dups, amount)
		}
	}
}

func runNoMap(b *testing.B, cb func(keys []string, data []*duplicates.Data) ([]string, [][]*duplicates.Data)) {
	run := func(dups int, amount int) {
		b.Run(fmt.Sprint(dups)+"dups_"+fmt.Sprint(amount)+"amount", func(b *testing.B) {
			keys, data := generateWithDups(n, dups, amount)
			b.ResetTimer()

			//execute
			var resKeys []string
			var resData [][]*duplicates.Data
			for i := 0; i < b.N; i++ {
				resKeys, resData = cb(keys, data)
			}
			globalKeys = resKeys
			globalDataSlices = resData
		})
	}

	run(0, 0)

	for _, dups := range []int{1, 5, 10, 50, 100} {
		for _, amount := range []int{2, 10, 50} {
			run(dups, amount)
		}
	}
}

func generateWithDups(n int, dups int, amount int) (keys []string, data []*duplicates.Data) {
	keys = make([]string, n)
	data = make([]*duplicates.Data, n)

	for i := 0; i < n; i++ {
		keys[i] = uuid.New().String()
		data[i] = &duplicates.Data{NumberA: randomInt64(), NumberB: randomInt64()}
	}

	rand.Seed(uint64(time.Now().UnixNano()))

	var existingPositions []int
	for range dups {
		dupKey := uuid.New().String()
		for range amount {
			pos := rand.Intn(n)
			//check for collisions
			for {
				posFound := false
				for _, existingPos := range existingPositions {
					if existingPos == pos {
						posFound = true
						break
					}
				}
				if posFound {
					pos = rand.Intn(n)
				} else {
					existingPositions = append(existingPositions, pos)
					break
				}
			}

			keys[pos] = dupKey
		}

	}

	return
}

func randomInt64() int64 {
	rand.Seed(uint64(time.Now().UnixNano()))
	return rand.Int63() // Int63 returns a non-negative pseudo-random 63-bit integer as an int64
}
