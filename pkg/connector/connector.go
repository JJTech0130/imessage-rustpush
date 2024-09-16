package connector

import (
	"context"
	"go.mau.fi/util/configupgrade"
	"maunium.net/go/mautrix/bridgev2"
	"maunium.net/go/mautrix/bridgev2/database"
)

type IMessageConnector struct {
	br *bridgev2.Bridge
}

// CreateLogin implements bridgev2.NetworkConnector.
func (i *IMessageConnector) CreateLogin(ctx context.Context, user *bridgev2.User, flowID string) (bridgev2.LoginProcess, error) {
	panic("unimplemented")
}

// GetCapabilities implements bridgev2.NetworkConnector.
func (i *IMessageConnector) GetCapabilities() *bridgev2.NetworkGeneralCapabilities {
	panic("unimplemented")
}

// GetConfig implements bridgev2.NetworkConnector.
func (i *IMessageConnector) GetConfig() (example string, data any, upgrader configupgrade.Upgrader) {
	panic("unimplemented")
}

// GetDBMetaTypes implements bridgev2.NetworkConnector.
func (i *IMessageConnector) GetDBMetaTypes() database.MetaTypes {
	panic("unimplemented")
}

// GetLoginFlows implements bridgev2.NetworkConnector.
func (i *IMessageConnector) GetLoginFlows() []bridgev2.LoginFlow {
	panic("unimplemented")
}

// GetName implements bridgev2.NetworkConnector.
func (i *IMessageConnector) GetName() bridgev2.BridgeName {
	panic("unimplemented")
}

func (im *IMessageConnector) Init(bridge *bridgev2.Bridge) {
	im.br = bridge
}

// LoadUserLogin implements bridgev2.NetworkConnector.
func (i *IMessageConnector) LoadUserLogin(ctx context.Context, login *bridgev2.UserLogin) error {
	panic("unimplemented")
}

func (im *IMessageConnector) Start(ctx context.Context) error {

	panic("unimplemented")
	return nil
}

var _ bridgev2.NetworkConnector = (*IMessageConnector)(nil)
