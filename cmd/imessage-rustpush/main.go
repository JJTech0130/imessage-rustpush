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
	cfg := rustpushgo.CreateRelayConfig("K7EW-redacted-PXZ3-Q6BY")
	conn := rustpushgo.Connect(cfg, rustpushgo.NewWrappedApsState(""))
	users := rustpushgo.Login("redacted@icloud.com", "redacted", cfg, conn)

	println("State: "+ conn.State().ToString())

	im := rustpushgo.NewIMessageClient(conn, users, cfg)
	
	println(im)
	//rustpushgo.Test()
	//return
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