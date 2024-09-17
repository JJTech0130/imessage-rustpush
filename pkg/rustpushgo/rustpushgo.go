package rustpushgo

import "github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo/out/rustpushgo"

/*
#cgo LDFLAGS: -lrustpushgo -ldl -lm
*/
import "C"

func Init() {
	rustpushgo.Add(1, 1)
}