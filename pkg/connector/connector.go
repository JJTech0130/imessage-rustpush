package connector

import (
	"context"

	"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
	"go.mau.fi/util/configupgrade"
	"maunium.net/go/mautrix/bridgev2"
	"maunium.net/go/mautrix/bridgev2/database"
	//"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
)

type IMessageConnector struct {
	br *bridgev2.Bridge
}

func (im *IMessageConnector) GetCapabilities() *bridgev2.NetworkGeneralCapabilities {
	return &bridgev2.NetworkGeneralCapabilities{}
}

func (im *IMessageConnector) GetConfig() (example string, data any, upgrader configupgrade.Upgrader) {
	return "", nil, configupgrade.NoopUpgrader
}

func (im *IMessageConnector) GetDBMetaTypes() database.MetaTypes {
	return database.MetaTypes{
		Portal:   nil,
		Ghost:    nil,
		Message:  nil,
		Reaction: nil,
		UserLogin: func() any {
			return &UserLoginMetadata{}
		},
	}
}

type UserLoginMetadata struct {
	APSState    string `json:"aps_state"`
	IDSUsers    string `json:"ids_users"`
	IDSIdentity string `json:"ids_identity"`
	RelayCode   string `json:"relay_code"`
}

func (im *IMessageConnector) GetName() bridgev2.BridgeName {
	return bridgev2.BridgeName{
		DisplayName:      "iMessage",
		NetworkURL:       "https://support.apple.com/messages",
		NetworkIcon:      "mxc://maunium.net/tManJEpANASZvDVzvRvhILdX",
		NetworkID:        "imessage",
		BeeperBridgeType: "imessagego",
		DefaultPort:      29337,
	}
}

func (im *IMessageConnector) Init(bridge *bridgev2.Bridge) {
	im.br = bridge
}

func (im *IMessageConnector) LoadUserLogin(ctx context.Context, login *bridgev2.UserLogin) error {
	meta := login.Metadata.(*UserLoginMetadata)

	users := rustpushgo.NewWrappedIdsUsers(&meta.IDSUsers)
	identity := rustpushgo.NewWrappedIdsUserIdentity(&meta.IDSIdentity)
	initialAPSState := rustpushgo.NewWrappedApsState(&meta.APSState)
	cfg := rustpushgo.CreateRelayConfig(meta.RelayCode)

	login.Client = &IMessageClient{
		Main:            im,
		UserLogin: 	 login,
		config:          cfg,
		users:           users,
		identity:        identity,
		initialAPSState: initialAPSState,
	}

	return nil
}

func (im *IMessageConnector) Start(ctx context.Context) error {
	return nil
}

var _ bridgev2.NetworkConnector = (*IMessageConnector)(nil)
