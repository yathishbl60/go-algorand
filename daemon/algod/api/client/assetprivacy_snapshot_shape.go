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
	"bytes"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
)

func nativePrivacyAddressShape(value string, nonzero bool) bool {
	address, err := basics.UnmarshalChecksumAddress(value)
	return err == nil && (!nonzero || !address.IsZero())
}

func nativePrivacySnapshotShape(value model.NativeAssetPrivacySnapshot) bool {
	if !nativePrivacyAddressShape(value.Creator, true) || !nativePrivacyAddressShape(value.Manager, false) || value.PrivateSupply > value.Total {
		return false
	}
	if value.CreatorHolding != nil && *value.CreatorHolding > value.Total {
		return false
	}
	if value.Inventory != nil && (value.Inventory.Items == 0 || value.Inventory.Bytes < value.Inventory.Items) {
		return false
	}
	if !value.Exists {
		return nativePrivacyClosedShape(value)
	}
	return nativePrivacyLiveSnapshotShape(value)
}

func nativePrivacyLiveSnapshotShape(value model.NativeAssetPrivacySnapshot) bool {
	if value.Closure != nil {
		return false
	}
	if value.Policy == nil {
		return value.Retirement == nil && value.Inventory == nil && value.PrivateSupply == 0
	}
	if value.Inventory == nil || !nativePrivacyPolicyShape(*value.Policy, value.Round) {
		return false
	}
	return nativePrivacyRetiredShape(value)
}

func nativePrivacyRetiredShape(value model.NativeAssetPrivacySnapshot) bool {
	marker := value.Retirement
	if marker == nil {
		return true
	}
	return marker.Epoch > 0 && nativePrivacyAddressShape(marker.Manager, true) && marker.Manager == value.Manager && marker.CreatedRound >= value.Policy.CreatedRound && marker.CreatedRound <= value.Round &&
		bytes.Equal(marker.PolicyId, value.Policy.PolicyId) && bytes.Equal(marker.Suite, value.Policy.Suite) && value.PrivateSupply == 0 &&
		value.CreatorHolding != nil && *value.CreatorHolding == value.Total
}

func nativePrivacyClosedShape(value model.NativeAssetPrivacySnapshot) bool {
	closed := value.Closure
	if closed == nil || value.Policy != nil || value.Retirement != nil || value.Inventory == nil || value.CreatorHolding != nil || value.PrivateSupply != 0 {
		return false
	}
	return nativePrivacyAddressShape(closed.Manager, true) && closed.Creator == value.Creator && closed.Manager == value.Manager && closed.Total == value.Total && closed.Epoch > 0 && closed.CreatedRound <= value.Round &&
		len(closed.PolicyId) == 64 && len(closed.Suite) == 64 && len(closed.Group) == 64 && len(closed.DestroyTransaction) == 32
}
