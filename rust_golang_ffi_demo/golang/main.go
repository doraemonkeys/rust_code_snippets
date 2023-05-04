package main

// #cgo CFLAGS: -I./cgo
// #cgo LDFLAGS: -L./ -lrustdemo
// #include "rustdemo.h"
import "C"
import (
	"fmt"
)

func main() {
	name := C.CString("golang")
	result := C.GoString(C.rustdemo(name))
	fmt.Printf("result: %s\n", result)
}

// unsafe 调用 dll
/*
package main

import (
	"C"
	"fmt"
	"syscall"
	"unsafe"
)

// char* rustdemo(char* name);

func main() {
	dllPath := "rustdemo.dll"
	rustdemo := syscall.NewLazyDLL(dllPath).NewProc("rustdemo")
	name := "golang"
	name_byte_ptr, err := syscall.BytePtrFromString(name)
	if err != nil {
		panic(err)
	}
	result_ptr, _, _ := rustdemo.Call(uintptr(unsafe.Pointer(name_byte_ptr)))
	// result := readCByte(result_ptr)
	// fmt.Printf("result: %s\n", result)
	result := C.GoString((*C.char)(unsafe.Pointer(result_ptr)))
	fmt.Printf("result: %s\n", result)
}


func readCByte(ptr uintptr) []byte {
	var result []byte
	for {
		b := (*byte)(unsafe.Pointer(ptr))
		if *b == 0 {
			break
		}
		result = append(result, *b)
		ptr++
	}
	return result
}

*/
