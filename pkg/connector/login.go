package connector

import (
	"context"

	"maunium.net/go/mautrix/bridgev2"
	//"maunium.net/go/mautrix/bridgev2/database"

	"github.com/JJTech0130/imessage-rustpush/pkg/rustpushgo"
)

func (im *IMessageConnector) GetLoginFlows() []bridgev2.LoginFlow {
	return []bridgev2.LoginFlow{{
		Name:        "Apple ID",
		Description: "Login with your Apple ID",
		ID:          "appleid",
	}, {
		Name:        "Phone Number",
		Description: "Login with your phone number",
		ID:          "phonenumber",
	}}
}

type AppleIDLogin struct {
	User *bridgev2.User
	conn *rustpushgo.WrappedApsConnection
	cfg *rustpushgo.WrappedOsConfig
	usersAndIdentity *rustpushgo.IdsUsersWithIdentityRecord
}

// Cancel implements bridgev2.LoginProcessUserInput.
func (a *AppleIDLogin) Cancel() {
	panic("unimplemented")
}

var RegistrationCodeStep = &bridgev2.LoginStep{
	Type: bridgev2.LoginStepTypeUserInput,
	StepID: "imessage.registration_code",
	UserInputParams: &bridgev2.LoginUserInputParams{Fields: []bridgev2.LoginInputDataField{{
		ID:       "code",
		Name:    "Registration Code",
	}}},
}

var UsernamePasswordStep = &bridgev2.LoginStep{
	Type: bridgev2.LoginStepTypeUserInput,
	StepID: "imessage.appleid.username_and_password",
	UserInputParams: &bridgev2.LoginUserInputParams{Fields: []bridgev2.LoginInputDataField{{
		Type: 	  bridgev2.LoginInputFieldTypeEmail,
		ID:       "username",
		Name:    "Apple ID",
	}, {
		Type: 	  bridgev2.LoginInputFieldTypePassword,
		ID:       "password",
		Name:    "Password",
	}}},
}

var TwoFactorStep = &bridgev2.LoginStep{
	Type: bridgev2.LoginStepTypeUserInput,
	StepID: "imessage.appleid.two_factor",
	UserInputParams: &bridgev2.LoginUserInputParams{Fields: []bridgev2.LoginInputDataField{{
		ID:       "code",
		Name:    "Two Factor Code",
	}}},
}

func (a *AppleIDLogin) Start(ctx context.Context) (*bridgev2.LoginStep, error) {
	return RegistrationCodeStep, nil
}

func (a *AppleIDLogin) SubmitUserInput(ctx context.Context, input map[string]string) (*bridgev2.LoginStep, error) {
	if a.cfg == nil {
		if code, ok := input["code"]; ok {
			a.cfg = rustpushgo.CreateRelayConfig(code)
		} else {
			return RegistrationCodeStep, nil
		}
	}
	if a.conn == nil {
		// We connect to APNs here because we need it to login
		a.conn = rustpushgo.Connect(a.cfg, rustpushgo.NewWrappedApsState(nil))
	}
	if a.usersAndIdentity == nil {
		if username, ok := input["username"]; ok {
			if password, ok := input["password"]; ok {
				result := rustpushgo.Login(username, password, a.cfg, a.conn)
				a.usersAndIdentity = &result
			} else {
				return UsernamePasswordStep, nil
			}
		} else {
			return UsernamePasswordStep, nil
		}
	}

	panic("unimplemented")
	
	// a.User.NewLogin(ctx, &database.UserLogin{
	// 	ID: 
	// })
}

var _ bridgev2.LoginProcessUserInput = (*AppleIDLogin)(nil)

func (im *IMessageConnector) CreateLogin(ctx context.Context, user *bridgev2.User, flowID string) (bridgev2.LoginProcess, error) {
	if flowID == "appleid" {
		return &AppleIDLogin{
			User: user,
		}, nil
	}
	panic("unimplemented")
}