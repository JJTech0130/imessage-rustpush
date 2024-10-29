package connector

import (
	"fmt"
	"strings"

	"maunium.net/go/mautrix/bridgev2/networkid"
)

// in the form tel:+10000000000 or mailto:example@example.com
func makeUserID(raw string) networkid.UserID {
	// Check if the raw string is an email
	if strings.Contains(raw, "@") {
		// Check if it already has the mailto: prefix
		if strings.HasPrefix(raw, "mailto:") {
			return networkid.UserID(raw)
		} else {
			return networkid.UserID(fmt.Sprintf("mailto:%s", raw))
		}
	} else {
		// Check if it already has the tel: prefix
		if strings.HasPrefix(raw, "tel:") {
			return networkid.UserID(raw)
		} else {
			return networkid.UserID(fmt.Sprintf("tel:%s", raw))
		}
	}
}

func makePortalID(e164Phone string) networkid.PortalID {
	return networkid.PortalID(strings.TrimLeft(e164Phone, "+"))
}

// Start with D:numbers D:18319046097
func makeUserLoginID(accountSID, phoneSID string) networkid.UserLoginID {
	return networkid.UserLoginID(fmt.Sprintf("%s:%s", accountSID, phoneSID))
}
