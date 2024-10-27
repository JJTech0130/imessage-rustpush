package connector

import (
	"context"
	"sync/atomic"

	"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
	"github.com/rs/zerolog/log"
	"maunium.net/go/mautrix/bridgev2"
	"maunium.net/go/mautrix/bridgev2/networkid"
)

type IMessageClient struct {
	Main   *IMessageConnector
	Client *rustpushgo.Client

	config          *rustpushgo.WrappedOsConfig
	users           *rustpushgo.WrappedIdsUsers
	identity        *rustpushgo.WrappedIdsUserIdentity
	initialAPSState *rustpushgo.WrappedApsState

	Connection *rustpushgo.WrappedApsConnection

	stopLoops atomic.Pointer[context.CancelFunc]
}

func (i *IMessageClient) updateUsersLoop(ctx context.Context) {
	// TODO: I don't think this is correct
	for {
		select {
		case <-ctx.Done():
			return
		default:
			i.users = i.Client.GetUpdatedUsers()
		}
		log.Debug().Any("users", i.users.ToString()).Msg("Got updated users")
	}
}

func (i *IMessageClient) startLoops() {
	ctx, cancel := context.WithCancel(context.Background())
	oldStop := i.stopLoops.Swap(&cancel)
	if oldStop != nil {
		(*oldStop)()
	}
	go i.updateUsersLoop(ctx)
}

func (i *IMessageClient) Connect(ctx context.Context) error {
	i.Connection = rustpushgo.Connect(i.config, i.initialAPSState)
	i.Client = rustpushgo.NewClient(i.Connection, i.users, i.identity, i.config)

	i.startLoops()
	return nil
}

func (i *IMessageClient) Disconnect() {
	if stopLoops := i.stopLoops.Swap(nil); stopLoops != nil {
		(*stopLoops)()
	}
	if cli := i.Client; cli != nil {
		//cli.Disconnect()
		i.Client = nil
	}
}

// GetCapabilities implements bridgev2.NetworkAPI.
func (i *IMessageClient) GetCapabilities(ctx context.Context, portal *bridgev2.Portal) *bridgev2.NetworkRoomCapabilities {
	panic("unimplemented")
}

// GetChatInfo implements bridgev2.NetworkAPI.
func (i *IMessageClient) GetChatInfo(ctx context.Context, portal *bridgev2.Portal) (*bridgev2.ChatInfo, error) {
	panic("unimplemented")
}

// GetUserInfo implements bridgev2.NetworkAPI.
func (i *IMessageClient) GetUserInfo(ctx context.Context, ghost *bridgev2.Ghost) (*bridgev2.UserInfo, error) {
	panic("unimplemented")
}

// HandleMatrixMessage implements bridgev2.NetworkAPI.
func (i *IMessageClient) HandleMatrixMessage(ctx context.Context, msg *bridgev2.MatrixMessage) (message *bridgev2.MatrixMessageResponse, err error) {
	panic("unimplemented")
}

// IsLoggedIn implements bridgev2.NetworkAPI.
func (i *IMessageClient) IsLoggedIn() bool {
	return i.Client != nil // TODO: Improve this?
}

// IsThisUser implements bridgev2.NetworkAPI.
func (i *IMessageClient) IsThisUser(ctx context.Context, userID networkid.UserID) bool {
	return false // TODO
}

// LogoutRemote implements bridgev2.NetworkAPI.
func (i *IMessageClient) LogoutRemote(ctx context.Context) {
	panic("unimplemented")
}

var _ bridgev2.NetworkAPI = (*IMessageClient)(nil)
