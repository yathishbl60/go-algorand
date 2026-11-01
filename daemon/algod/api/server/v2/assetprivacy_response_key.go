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

package v2

import (
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func nativePrivacyKeyResponse(value query.KeySnapshot) (model.NativeAssetPrivacyKeySnapshot, error) {
	if (value.Record == nil) != (value.Witness == nil) {
		return model.NativeAssetPrivacyKeySnapshot{}, query.ErrState
	}
	result := model.NativeAssetPrivacyKeySnapshot{Round: value.Round, AssetId: value.Asset, PolicyId: value.PolicyID[:],
		Account: basics.Address(value.Account).String(), Role: uint64(value.Role), PolicyAllowed: value.PolicyAllowed, Assigned: value.Assigned,
		KeyRoot: value.Root.Digest[:], KeyCount: value.Count, Registered: value.Record != nil, Witness: nativePrivacyMembershipResponse(value.Witness)}
	if value.Record != nil {
		result.Record = &model.NativeAssetPrivacyKeyRecord{Epoch: value.Record.Epoch, CreatedRound: value.Record.Created, PublicKey: value.Record.PublicKey}
	}
	return result, nil
}
