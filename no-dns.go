package main

import (
	"fmt"
	"proxy"
)

func main() {
	fmt.Println("No DNS proxy starting!")
	proxy.RunDnsProxy()
}
