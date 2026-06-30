package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: llt/target/debug/libllt.dylib
#include "llt/include/llt.h"
*/
import "C"
import "fmt"

func main() {
	res := C.add(2, 3)
	fmt.Println(res)
}
