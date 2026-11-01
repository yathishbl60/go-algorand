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
	"fmt"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
)

func (client *NativePrivacyClient) Key(ctx context.Context, asset basics.AssetIndex, account basics.Address, role NativePrivacyRole, round *basics.Round) (model.NativeAssetPrivacyKeySnapshot, error) {
	if asset == 0 || account.IsZero() || role < NativePrivacyRecipientView || role > NativePrivacyManagerAuthority {
		return model.NativeAssetPrivacyKeySnapshot{}, ErrNativePrivacyRequest
	}
	var result model.NativeAssetPrivacyKeySnapshot
	path := nativePrivacyPath(asset, fmt.Sprintf("/accounts/%s/keys/%d", account.String(), role))
	if err := client.get(ctx, path, nativePrivacyRoundQuery(round), &result); err != nil {
		return model.NativeAssetPrivacyKeySnapshot{}, err
	}
	if err := nativePrivacyScope(asset, round, result.Round, result.AssetId); err != nil {
		return model.NativeAssetPrivacyKeySnapshot{}, err
	}
	if result.Account != account.String() || result.Role != uint64(role) || len(result.PolicyId) != 64 || len(result.KeyRoot) != 64 {
		return model.NativeAssetPrivacyKeySnapshot{}, ErrNativePrivacyResponse
	}
	if err := nativePrivacyKeyShape(result); err != nil {
		return model.NativeAssetPrivacyKeySnapshot{}, err
	}
	return result, nil
}
