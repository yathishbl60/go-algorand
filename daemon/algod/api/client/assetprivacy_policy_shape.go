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
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func nativePrivacyPolicyShape(value model.NativeAssetPrivacyPolicy, round uint64) bool {
	if len(value.PolicyId) != 64 || len(value.Suite) != 64 || value.CreatedRound > round || value.Modes > 255 ||
		value.Capabilities > 255 || value.InitialCapabilities > 255 || value.Disclosures > 255 {
		return false
	}
	record := policy.Record{Policy: policy.Policy{Modes: policy.Modes(value.Modes), Capabilities: policy.Capabilities(value.Capabilities),
		Disclosures: policy.Disclosures(value.Disclosures)}, InitialCapabilities: policy.Capabilities(value.InitialCapabilities), Created: value.CreatedRound}
	copy(record.Suite[:], value.Suite)
	return record.Validate() == nil
}

func nativePrivacyAuthorizationShape(value model.NativeAssetPrivacyAuthorization) bool {
	if len(value.PolicyId) != 64 || len(value.KeyRoot) != 64 || len(value.EligibilityRoot) != 64 || value.KeyCount > 1<<32 || value.EligibilityCount > 1<<32 {
		return false
	}
	if !nativePrivacyAddressShape(value.Manager, false) || !nativePrivacyAddressShape(value.Freeze, false) || !nativePrivacyAddressShape(value.Clawback, false) {
		return false
	}
	return nativePrivacyTargetsShape(value.ViewTargets, value.Round)
}

func nativePrivacyTargetsShape(value *model.NativeAssetPrivacyViewTargets, round uint64) bool {
	if value == nil {
		return true
	}
	if value.Epoch == 0 || value.UpdatedRound > round || value.Flags & ^uint64(3) != 0 {
		return false
	}
	return nativePrivacyTargetAddress(value.Auditor, value.Flags&1 != 0) && nativePrivacyTargetAddress(value.Controller, value.Flags&2 != 0)
}

func nativePrivacyTargetAddress(value *string, required bool) bool {
	if !required {
		return value == nil
	}
	return value != nil && nativePrivacyAddressShape(*value, true)
}
