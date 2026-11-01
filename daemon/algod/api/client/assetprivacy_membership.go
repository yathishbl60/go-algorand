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
)

func nativePrivacyMembershipShape(path *model.NativeAssetPrivacyMembershipPath, root []byte, count uint64) bool {
	return path != nil && bytes.Equal(path.Root, root) && path.Count == count && nativePrivacyPathShape(path.Root, path.Leaf, path.Siblings, path.Count, path.Index)
}

func nativePrivacyKeyShape(value model.NativeAssetPrivacyKeySnapshot) error {
	if value.KeyCount > 1<<32 || value.Assigned && !value.PolicyAllowed {
		return ErrNativePrivacyResponse
	}
	if value.Registered != (value.Record != nil) || value.Registered != (value.Witness != nil) {
		return ErrNativePrivacyResponse
	}
	if !value.Registered {
		return nil
	}
	record := value.Record
	if record.Epoch == 0 || record.CreatedRound > value.Round || !nativePrivacyMembershipShape(value.Witness, value.KeyRoot, value.KeyCount) {
		return ErrNativePrivacyResponse
	}
	if len(record.PublicKey) != nativePrivacyKeySize(value.Role) {
		return ErrNativePrivacyResponse
	}

	return nil
}

func nativePrivacyEligibilityShape(value model.NativeAssetPrivacyEligibilitySnapshot) error {
	if value.EligibilityCount > 1<<32 || value.Member != (value.Record != nil) || value.Member != (value.Witness != nil) {
		return ErrNativePrivacyResponse
	}
	if value.Marker != nil && (value.Marker.Epoch == 0 || value.Marker.UpdatedRound > value.Round) {
		return ErrNativePrivacyResponse
	}
	if !value.Member {
		return nil
	}
	record := value.Record
	if record.Revision == 0 || record.UpdatedRound > value.Round || record.Frozen != value.Frozen {
		return ErrNativePrivacyResponse
	}
	if !nativePrivacyMembershipShape(value.Witness, value.EligibilityRoot, value.EligibilityCount) {
		return ErrNativePrivacyResponse
	}
	return nil
}
