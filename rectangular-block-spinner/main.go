// https://www.codingame.com/ide/puzzle/rectangular-block-spinner
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type step struct {
	startX, startY int
	rowX, rowY     int
	colX, colY     int
}

func getStep(size, angle int) step {
	var s step
	switch (angle / 45) % 8 {
	case 1:
		s = step{0, size - 1, 1, -1, 1, 1}
	case 3:
		s = step{size - 1, size*2 - 2, -1, -1, 1, -1}
	case 5:
		s = step{size - 1, size - 1, -1, 1, -1, -1}
	case 7:
		s = step{size - 1, 0, 1, 1, -1, 1}
	}
	return s
}

func main() {
	//f, _ := os.Open("./t4")
	//scanner := bufio.NewScanner(f)
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 1000000), 1000000)

	var size int
	scanner.Scan()
	fmt.Sscan(scanner.Text(), &size)

	var angle int
	scanner.Scan()
	fmt.Sscan(scanner.Text(), &angle)

	out := make([][]string, 2*size-1)
	for i := range out {
		for j := 0; j < 2*size-1; j++ {
			out[i] = append(out[i], " ")
		}
	}

	step := getStep(size, angle)

	for i := 0; i < size; i++ {
		scanner.Scan()
		line := strings.Split(scanner.Text(), " ")
		for j, l := range line {
			out[step.startY+(j*step.rowY)][step.startX+(j*step.rowX)] = l
		}
		step.startX += step.colX
		step.startY += step.colY
	}

	for i := range out {
		fmt.Println(strings.Join(out[i], ""))
	}
}
