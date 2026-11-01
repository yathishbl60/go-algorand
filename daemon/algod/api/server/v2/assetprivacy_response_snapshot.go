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

func nativePrivacySnapshotResponse(value query.Snapshot) model.NativeAssetPrivacySnapshot {
	response := model.NativeAssetPrivacySnapshot{Round: value.Round, AssetId: value.Asset, Exists: value.Exists,
		Creator: basics.Address(value.Creator).String(), Manager: basics.Address(value.Manager).String(),
		Total: value.Total, PrivateSupply: value.Private, Policy: nativePrivacyPolicyResponse(value)}
	if value.HasCreatorHolding {
		response.CreatorHolding = &value.CreatorHolding
	}
	if value.Inventory != nil {
		response.Inventory = &model.NativeAssetPrivacyInventory{Items: value.Inventory.Items, Bytes: value.Inventory.Bytes}
	}
	response.Retirement = nativePrivacyRetirementResponse(value.Retired)
	response.Closure = nativePrivacyClosureResponse(value.Closure)
	return response
}

func nativePrivacyPolicyResponse(value query.Snapshot) *model.NativeAssetPrivacyPolicy {
	if value.Policy == nil {
		return nil
	}
	record := *value.Policy
	return &model.NativeAssetPrivacyPolicy{PolicyId: value.PolicyID[:], Suite: record.Suite[:], CreatedRound: record.Created,
		Modes: uint64(record.Policy.Modes), Capabilities: uint64(record.Policy.Capabilities),
		InitialCapabilities: uint64(record.InitialCapabilities), Disclosures: uint64(record.Policy.Disclosures)}
}
