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

package client

import (
	"context"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/daemon/algod/api/spec/common"
)

// GenesisHash reads the configured RPC network identity, not a finality proof.
func (client *NativePrivacyClient) GenesisHash(ctx context.Context) (crypto.Digest, error) {
	var response common.Version
	if err := client.get(ctx, "versions", nil, &response); err != nil {
		return crypto.Digest{}, err
	}
	if len(response.GenesisHash) != 32 {
		return crypto.Digest{}, ErrNativePrivacyResponse
	}
	var hash crypto.Digest
	copy(hash[:], response.GenesisHash)
	if hash == (crypto.Digest{}) {
		return crypto.Digest{}, ErrNativePrivacyResponse
	}
	return hash, nil
}
