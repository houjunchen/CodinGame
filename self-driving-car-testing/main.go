// https://www.codingame.com/ide/puzzle/self-driving-car-testing
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func expandCmds(cmds []string) string {
	ret := ""
	for _, c := range cmds {
		times, _ := strconv.Atoi(c[:len(c)-1])
		dir := c[len(c)-1]
		ret += strings.Repeat(string(dir), times)
	}
	return ret
}

func main() {
	//scanner := bufio.NewScanner(os.Stdin)
	f, _ := os.Open("./r4")
	scanner := bufio.NewScanner(f)
	scanner.Buffer(make([]byte, 1000000), 1000000)

	var n int
	scanner.Scan()
	fmt.Sscan(scanner.Text(), &n)

	scanner.Scan()
	cmds := strings.Split(scanner.Text(), ";")

	pos, _ := strconv.Atoi(cmds[0])
	cmd := expandCmds(cmds[1:])
	cmdIndex := 0

	for i := 0; i < n; i++ {
		scanner.Scan()
		line := strings.Split(scanner.Text(), ";")
		count, _ := strconv.Atoi(line[0])
		road := line[1]
		for j := 0; j < count; j++ {
			if string(cmd[cmdIndex]) == "R" {
				pos += 1
			} else if string(cmd[cmdIndex]) == "L" {
				pos -= 1
			}
			fmt.Printf("%s#%s\n", road[:pos-1], road[pos:])
			cmdIndex++
		}
	}
}
