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
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func nativePrivacyAuthorizationResponse(value query.AuthorizationSnapshot) model.NativeAssetPrivacyAuthorization {
	return model.NativeAssetPrivacyAuthorization{Round: value.Round, AssetId: value.Asset, PolicyId: value.PolicyID[:],
		Manager: basics.Address(value.Manager).String(), Freeze: basics.Address(value.Freeze).String(), Clawback: basics.Address(value.Clawback).String(),
		DefaultFrozen: value.DefaultFrozen, KeyRoot: value.Keys.Digest[:], KeyCount: value.KeyCount,
		EligibilityRoot: value.Eligibility.Digest[:], EligibilityCount: value.EligibilityCount, ViewTargets: nativePrivacyViewTargetsResponse(value.Targets)}
}

func nativePrivacyViewTargetsResponse(value *disclosure.Record) *model.NativeAssetPrivacyViewTargets {
	if value == nil {
		return nil
	}
	result := &model.NativeAssetPrivacyViewTargets{Flags: uint64(value.Flags), Epoch: value.Epoch, UpdatedRound: value.Updated}
	if value.Flags&disclosure.AuditorRequired != 0 {
		address := basics.Address(value.Auditor).String()
		result.Auditor = &address
	}
	if value.Flags&disclosure.ControllerRequired != 0 {
		address := basics.Address(value.Controller).String()
		result.Controller = &address
	}
	return result
}
