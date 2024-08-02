package duplicates_test

import (
	"duplicates"
	"testing"
)

func Test_Naive(t *testing.T) {
	runDupTest(t, duplicates.DuplicatesNaive)
}

func Test_NaiveIndices(t *testing.T) {
	runDupTest(t, duplicates.DuplicatesNaiveIndices)
}

func Test_Avoid(t *testing.T) {
	runDupTest(t, duplicates.DuplicatesAvoidFirstMap)
}

func runDupTest(t *testing.T, cb func(keys []string, data []*duplicates.Data) ([]string, []*duplicates.Data, map[string][]*duplicates.Data)) {
	tests := []struct {
		name             string
		keys             []string
		data             []*duplicates.Data
		expectedNonDupK  int
		expectedNonDupD  int
		expectedDupCount map[string]int
	}{
		{
			name:             "Empty Input",
			keys:             []string{},
			data:             []*duplicates.Data{},
			expectedNonDupK:  0,
			expectedNonDupD:  0,
			expectedDupCount: make(map[string]int),
		},
		{
			name:             "All Unique",
			keys:             []string{"a", "b", "c"},
			data:             []*duplicates.Data{{1, 2}, {3, 4}, {5, 6}},
			expectedNonDupK:  3,
			expectedNonDupD:  3,
			expectedDupCount: make(map[string]int),
		},
		{
			name:             "All Duplicates",
			keys:             []string{"a", "a", "b", "b", "b"},
			data:             []*duplicates.Data{{1, 2}, {1, 2}, {3, 4}, {5, 6}, {7, 8}},
			expectedNonDupK:  0,
			expectedNonDupD:  0,
			expectedDupCount: map[string]int{"a": 2, "b": 3},
		},
		{
			name:             "Mixed Content",
			keys:             []string{"a", "a", "b", "c", "c", "c"},
			data:             []*duplicates.Data{{1, 2}, {1, 2}, {3, 4}, {5, 6}, {7, 8}, {9, 10}},
			expectedNonDupK:  1,
			expectedNonDupD:  1,
			expectedDupCount: map[string]int{"a": 2, "c": 3},
		},
		{
			name:             "Duplicates with Different Data",
			keys:             []string{"a", "a", "b"},
			data:             []*duplicates.Data{{1, 2}, {3, 4}, {5, 6}},
			expectedNonDupK:  1,
			expectedNonDupD:  1,
			expectedDupCount: map[string]int{"a": 2},
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			nonDuplicateKeys, nonDuplicateData, duplicates := cb(tc.keys, tc.data)
			if len(nonDuplicateKeys) != tc.expectedNonDupK || len(nonDuplicateData) != tc.expectedNonDupD {
				t.Errorf("Test %s failed: expected %d nonDuplicateKeys and %d nonDuplicateData, got %d and %d",
					tc.name, tc.expectedNonDupK, tc.expectedNonDupD, len(nonDuplicateKeys), len(nonDuplicateData))
			}
			for key, expectedCount := range tc.expectedDupCount {
				if len(duplicates[key]) != expectedCount {
					t.Errorf("Test %s failed: for key %s expected %d duplicates, got %d",
						tc.name, key, expectedCount, len(duplicates[key]))
				}
			}
		})
	}
}
