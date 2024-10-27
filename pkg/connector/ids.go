package connector

import (
	"fmt"
	"strings"

	"maunium.net/go/mautrix/bridgev2/networkid"
)

func makeUserID(e164Phone string) networkid.UserID {
	return networkid.UserID(strings.TrimLeft(e164Phone, "+"))
}

func makePortalID(e164Phone string) networkid.PortalID {
	return networkid.PortalID(strings.TrimLeft(e164Phone, "+"))
}

// Start with D:numbers D:18319046097
func makeUserLoginID(accountSID, phoneSID string) networkid.UserLoginID {
	return networkid.UserLoginID(fmt.Sprintf("%s:%s", accountSID, phoneSID))
}