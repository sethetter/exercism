package listops

func (l []int) Foldl(f func(int, int) int, v int) int {
	for _, x := range l {
		v = f(x, v)
	}
	return v
}
