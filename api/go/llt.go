package llt

import (
	"encoding/base64"
	"fmt"
	"os"
	"unsafe"

	"github.com/ebitengine/purego"
)

type MathInput struct {
	A int32
	B int32
}

type MathOutput struct {
	Result int32
	Error  int32
}

func Init() {
	data, _ := base64.StdEncoding.DecodeString(ContentDynLibLltBase64)

	tmp := os.TempDir() + "/libllt.so"
	_ = os.WriteFile(tmp, data, 0755)

	lib, err := purego.Dlopen(tmp, purego.RTLD_NOW|purego.RTLD_GLOBAL)

	if err != nil {
		panic(err)
	}

	var mathAdd func(in, out unsafe.Pointer)
	purego.RegisterLibFunc(&mathAdd, lib, "math_add")

	in := MathInput{A: 3, B: 4}
	var out MathOutput

	mathAdd(unsafe.Pointer(&in), unsafe.Pointer(&out))

	fmt.Printf("result=%d error=%d\n", out.Result, out.Error)
}
