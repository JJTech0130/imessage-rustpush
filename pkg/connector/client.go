package connector

import (
	"context"
	"fmt"
	"sync/atomic"

	"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
	"github.com/rs/zerolog/log"
	"go.mau.fi/util/ptr"
	"maunium.net/go/mautrix/bridgev2"
	"maunium.net/go/mautrix/bridgev2/networkid"
	"maunium.net/go/mautrix/event"
)

type IMessageClient struct {
	Main   *IMessageConnector
	Client *rustpushgo.Client

	UserLogin *bridgev2.UserLogin

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
			log.Debug().Msg("updateUsersLoop cancelled")
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
	if i.Connection == nil {
		i.Connection = rustpushgo.Connect(i.config, i.initialAPSState)
	}
	i.Client = rustpushgo.NewClient(i.Connection, i.users, i.identity, i.config)

	//i.startLoops()
	return nil
}

func (i *IMessageClient) Disconnect() {
	log.Debug().Msg("Disconnecting")
	if stopLoops := i.stopLoops.Swap(nil); stopLoops != nil {
		(*stopLoops)()
	}

	i.Client.Destroy()
	i.Client = nil
}

func (i *IMessageClient) getUserLoginID() networkid.UserLoginID {
	return networkid.UserLoginID(i.users.LoginId(0))
}

func (i *IMessageClient) getUserID() networkid.UserID {
	return networkid.UserID(i.Client.GetHandles()[0])
}

func (i *IMessageClient) ResolveIdentifier(ctx context.Context, identifier string, createChat bool) (*bridgev2.ResolveIdentifierResponse, error) {
	userID := makeUserID(identifier)
	portalID := networkid.PortalKey{
		ID:       networkid.PortalID(userID),
		Receiver: i.UserLogin.ID,
	}
	ghost, err := i.UserLogin.Bridge.GetGhostByID(ctx, userID)
	if err != nil {
		return nil, fmt.Errorf("failed to get ghost: %w", err)
	}
	portal, err := i.UserLogin.Bridge.GetPortalByKey(ctx, portalID)
	if err != nil {
		return nil, fmt.Errorf("failed to get portal: %w", err)
	}
	ghostInfo, err := i.GetUserInfo(ctx, ghost)
	if err != nil {
		return nil, fmt.Errorf("failed to get user info: %w", err)
	}
	portalInfo, _ := i.GetChatInfo(ctx, portal)
	return &bridgev2.ResolveIdentifierResponse{
		Ghost:    ghost,
		UserID:   userID,
		UserInfo: ghostInfo,
		Chat: &bridgev2.CreateChatResponse{
			Portal:     portal,
			PortalInfo: portalInfo,
			PortalKey:  portalID,
		},
	}, nil
}

// GetCapabilities implements bridgev2.NetworkAPI.
func (i *IMessageClient) GetCapabilities(ctx context.Context, portal *bridgev2.Portal) *bridgev2.NetworkRoomCapabilities {
	panic("unimplemented")
}

func (i *IMessageClient) GetChatInfo(ctx context.Context, portal *bridgev2.Portal) (*bridgev2.ChatInfo, error) {
	return &bridgev2.ChatInfo{
		Members: &bridgev2.ChatMemberList{
			IsFull: true,
			Members: []bridgev2.ChatMember{
				{
					EventSender: bridgev2.EventSender{
						IsFromMe: true,
						Sender:   i.getUserID(),
					},
					Membership: event.MembershipJoin,
					PowerLevel: ptr.Ptr(50),
				},
				{
					EventSender: bridgev2.EventSender{
						Sender: networkid.UserID(portal.ID),
					},
					Membership: event.MembershipJoin,
					PowerLevel: ptr.Ptr(50),
				},
			},
		},
	}, nil
}

func (i *IMessageClient) GetUserInfo(ctx context.Context, ghost *bridgev2.Ghost) (*bridgev2.UserInfo, error) {
	if len(i.Client.ValidateTargets([]string{string(ghost.ID)}, string(i.getUserID()))) == 0 {
		return nil, fmt.Errorf("user not found: %s", ghost.ID)
	}
	return &bridgev2.UserInfo{
		Identifiers: []string{string(ghost.ID)},
		Name:        ptr.Ptr(string(ghost.ID)),
	}, nil
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
var _ bridgev2.IdentifierResolvingNetworkAPI = (*IMessageClient)(nil)
