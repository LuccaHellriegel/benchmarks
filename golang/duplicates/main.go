package duplicates

// some random data
type Data struct {
	NumberA int64
	NumberB int64
}

func DuplicatesNaive(keys []string, data []*Data) (nonDuplicateKeys []string, nonDuplicateData []*Data, duplicates map[string][]*Data) {
	lenData := len(data)

	m := make(map[string][]*Data, lenData)

	for i, key := range keys {
		m[key] = append(m[key], data[i])
	}

	duplicates = make(map[string][]*Data)
	nonDuplicateKeys = make([]string, 0, lenData)
	nonDuplicateData = make([]*Data, 0, lenData)

	for key, val := range m {
		if len(val) == 1 {
			nonDuplicateKeys = append(nonDuplicateKeys, key)
			nonDuplicateData = append(nonDuplicateData, val[0])
		} else {
			duplicates[key] = val
		}
	}

	return
}

func DuplicatesNaiveIndices(keys []string, data []*Data) (nonDuplicateKeys []string, nonDuplicateData []*Data, duplicates map[string][]*Data) {
	lenData := len(data)

	m := make(map[string][]int, lenData)

	for i, key := range keys {
		m[key] = append(m[key], i)
	}

	duplicates = make(map[string][]*Data)
	nonDuplicateKeys = make([]string, 0, lenData)
	nonDuplicateData = make([]*Data, 0, lenData)

	for key, val := range m {
		if len(val) == 1 {
			nonDuplicateKeys = append(nonDuplicateKeys, key)
			nonDuplicateData = append(nonDuplicateData, data[val[0]])
		} else {
			for _, i := range val {
				duplicates[key] = append(duplicates[key], data[i])
			}
		}
	}

	return
}

func DuplicatesAvoidFirstMap(keys []string, data []*Data) (nonDuplicateKeys []string, nonDuplicateData []*Data, duplicates map[string][]*Data) {
	lenData := len(data)

	duplicates = make(map[string][]*Data)
	nonDuplicateKeys = make([]string, 0, lenData)
	nonDuplicateData = make([]*Data, 0, lenData)

	//by keeping a simple int map around, we avoid the whole first map allocations
	m := make(map[string]int, lenData)

	for i, key := range keys {
		existingIndex, exists := m[key]
		if exists {
			// -1 means more than one duplicate, so don't need to add the initial one again
			if existingIndex == -1 {
				duplicates[key] = append(duplicates[key], data[i])
			} else {
				m[key] = -1
				duplicates[key] = append(duplicates[key], data[existingIndex], data[i]) //order is important
			}
		} else {
			m[key] = i
		}
	}

	for key, val := range m {
		if val != -1 {
			nonDuplicateKeys = append(nonDuplicateKeys, key)
			nonDuplicateData = append(nonDuplicateData, data[val])
		}
	}

	return
}

// 1000x slower
func DuplicatesNoMap(keys []string, data []*Data) (resKeys []string, resData [][]*Data) {
	lenData := len(data)

	resKeys = make([]string, 0, lenData)
	resData = make([][]*Data, 0, lenData)

	for i, key := range keys {
		found := false
		for resI, resKey := range resKeys {
			if resKey == key {
				found = true
				resData[resI] = append(resData[resI], data[i])
			}
		}
		if !found {
			resKeys = append(resKeys, key)
			resData = append(resData, []*Data{data[i]})
		}

	}

	return
}
