package main

/*
#cgo LDFLAGS: -L./lib -lrustdemo
#include <stdlib.h>
#include "./lib/rustdemo.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	s := "Go say: Hello Rust"
	input := C.CString(s)
	defer C.free(unsafe.Pointer(input))
	o := C.rustdemo(input)
	output := C.GoString(o)
	fmt.Printf("%s\n", output)
}
