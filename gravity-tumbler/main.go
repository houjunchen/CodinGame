// https://www.codingame.com/ide/puzzle/gravity-tumbler
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	//f, _ := os.Open("./r1")
	//scanner := bufio.NewScanner(f)
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 1000000), 1000000)

	var width, height int
	scanner.Scan()
	fmt.Sscan(scanner.Text(), &width, &height)

	var count int
	scanner.Scan()
	fmt.Sscan(scanner.Text(), &count)

	tumbler := []string{}

	for i := 0; i < height; i++ {
		scanner.Scan()
		raster := scanner.Text()
		hashtagCount := strings.Count(raster, "#")
		row := fmt.Sprintf(
			"%s%s",
			strings.Repeat(".", width-hashtagCount),
			strings.Repeat("#", hashtagCount),
		)
		tumbler = append(tumbler, row)
	}
	if count%2 == 1 {
		for i := 0; i < width; i++ {
			for j := 0; j < height; j++ {
				fmt.Print(string(tumbler[j][i]))
			}
			fmt.Println("")
		}
	} else {
		for _, l := range tumbler {
			fmt.Println(l)
		}
	}
}
