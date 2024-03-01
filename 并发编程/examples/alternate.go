package main

import "fmt"

func worker(notifyCh chan int, id int) {
	for i := range notifyCh {
		fmt.Printf("worker %d: %d\n", id, i)
		notifyCh <- i + 1
	}
}

// 两个线程/协程交替打印数字
func main() {
	notifyCh := make(chan int)
	go worker(notifyCh, 1)
	notifyCh <- 0
	worker(notifyCh, 2)
}
