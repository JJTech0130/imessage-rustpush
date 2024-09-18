package main

import (
	//"maunium.net/go/mautrix/bridgev2/matrix/mxmain"

	//"github.com/JJTech0130/imessage-rustpush/pkg/connector"
	"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
)
var (
	Tag = "unknown"
	Commit = "unknown"
	BuildTime = "unknown"
)

func main() {
	rustpushgo.Init()
	cfg := rustpushgo.CreateRelayConfig("65RQ-redacted-75AS-EA3A")
	conn := rustpushgo.Connect(cfg)
	rustpushgo.Login("redacted@icloud.com", "redacted", cfg, conn)
	
	//rustpushgo.Test()
	return
	/*m := mxmain.BridgeMain{
		Name: "imessage-rustpush",
		Description: "An iMessage bridge based on rustpush",
		URL: "https://github.com/JJTech0130/imessage-rustpush",
		Version: Tag,
		Connector: &connector.IMessageConnector{},
	}
	m.InitVersion(Tag, Commit, BuildTime)
	m.Run()*/
}