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

package transactions

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/protocol"
)

// NativeAssetPrivacySizeAllowed checks a conservative signed encoding estimate.
// Admission must reject a false result; this is not an aggregate memory budget.
func (s SignedTxn) NativeAssetPrivacySizeAllowed(proto config.ConsensusParams) bool {
	if proto.MaxTxnBytesPerBlock <= 0 {
		return false
	}
	estimate := s.Msgsize()
	return estimate > 0 && estimate <= proto.MaxTxnBytesPerBlock
}

func (s SignedTxn) nativeAssetPrivacyFeeContribution(proto config.ConsensusParams) basics.Micros {
	if (s.Txn.Type != protocol.NativeAssetPrivacyTx && !s.Txn.IsNativeAssetPrivacyCreation()) || !proto.NativeAssetPrivacyExecutionConfigured() || !s.NativeAssetPrivacySizeAllowed(proto) {
		return 0
	}
	checks, err := s.nativeAssetPrivacyProofChecks()
	if err != nil {
		return 0
	}
	proofs := basics.MulSaturate(proto.NativeAssetPrivacyProofFeeFactor, basics.Micros(checks))
	bytes := basics.MulSaturate(proto.NativeAssetPrivacyByteFeeFactor, basics.Micros(s.GetEncodedLength()))
	return basics.AddSaturate(proofs, bytes)
}
