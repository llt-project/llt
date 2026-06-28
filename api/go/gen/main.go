//go:build ignore
// +build ignore

package main

import (
	"encoding/base64"
	"os"
)

func main() {
	data, err := os.ReadFile("libllt.so")
	if err != nil {
		data, err = os.ReadFile("libllt.dll")
		if err != nil {
			data, err = os.ReadFile("libllt.dylib")
			if err != nil {
				panic(err)
			}
		}
	}

	encoded := base64.StdEncoding.EncodeToString(data)

	out := "package llt\n\n" +
		"var ContentDynLibLltBase64 = `" + encoded + "`\n"

	err = os.WriteFile("llt.content.gen.go", []byte(out), 0644)
	if err != nil {
		panic(err)
	}
}
