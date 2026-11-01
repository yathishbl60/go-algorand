//go:build algorand_native_privacy

// Copyright (C) 2019-2026 Algorand Foundation Ltd.
// This file is part of go-algorand
//
// go-algorand is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// go-algorand is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with go-algorand.  If not, see <https://www.gnu.org/licenses/>.

package libgoal

import (
	"net/http"

	algodclient "github.com/algorand/go-algorand/daemon/algod/api/client"
)

// NativePrivacyQueries uses this client's configured algod URL and API token.
func (client *Client) NativePrivacyQueries(httpClient *http.Client) (*algodclient.NativePrivacyClient, error) {
	current, err := client.ensureAlgodClient()
	if err != nil {
		return nil, err
	}
	return current.NativePrivacyQueries(httpClient)
}
