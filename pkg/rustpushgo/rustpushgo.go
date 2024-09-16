package rustpushgo

import "github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo/out/rustpush"

/*
#cgo LDFLAGS: -lrustpushgo -ldl -lm
*/
import "C"

func Init() {
	rustpush.Add(1, 1)
}