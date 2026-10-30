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

package verify

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/algorand/go-algorand/internal/assetprivacy/witness"
)

func checkNativePrivacyBinding(group []transactions.SignedTxn, index int, limits proofbridge.Limits, proto config.ConsensusParams) error {
	digest, suite, err := nativePrivacyGroupBinding(group, index, proto)
	if err != nil {
		return err
	}
	if lifecyclebody.IsLifecycle(group[index].Txn.NativeAssetPrivacy) {
		return checkNativePrivacyLifecycleScheme(&group[index], proto)
	}
	if uint64(len(group[index].NativeAssetPrivacyProof)) > uint64(proto.MaxTxnBytesPerBlock) {
		return errNativePrivacyWitness
	}
	cap := min(limits.MaxBundleBytes, limits.MaxAllocationBytes)
	return witness.Check(group[index].NativeAssetPrivacyProof, witness.Binding{Profile: suite, Statement: digest},
		witness.Limits{MaxProofBytes: cap})
}
