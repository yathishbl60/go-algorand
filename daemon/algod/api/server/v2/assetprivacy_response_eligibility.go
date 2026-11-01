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

func nativePrivacyEligibilityResponse(value query.EligibilitySnapshot) (model.NativeAssetPrivacyEligibilitySnapshot, error) {
	if (value.Record == nil) != (value.Witness == nil) {
		return model.NativeAssetPrivacyEligibilitySnapshot{}, query.ErrState
	}
	result := model.NativeAssetPrivacyEligibilitySnapshot{Round: value.Round, AssetId: value.Asset, PolicyId: value.PolicyID[:],
		Account: basics.Address(value.Account).String(), Frozen: value.Frozen, Member: value.Record != nil,
		EligibilityRoot: value.Root.Digest[:], EligibilityCount: value.Count, Witness: nativePrivacyMembershipResponse(value.Witness)}
	if value.Marker != nil {
		result.Marker = &model.NativeAssetPrivacyControlRecord{Epoch: value.Marker.Epoch, UpdatedRound: value.Marker.Updated, Frozen: value.Marker.Frozen}
	}
	if value.Record != nil {
		result.Record = &model.NativeAssetPrivacyEligibilityRecord{Revision: value.Record.Revision, UpdatedRound: value.Record.Updated, Frozen: value.Record.Frozen}
	}
	return result, nil
}
