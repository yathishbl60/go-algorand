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

package config

import "github.com/algorand/go-algorand/config/bounds"

func checkSetNativeAssetPrivacyAllocBounds(p ConsensusParams) {
	if !p.EnableNativeAssetPrivacy || len(p.NativeAssetPrivacySuites) == 0 ||
		len(p.NativeAssetPrivacySuites) > 16 || p.MaxTxnBytesPerBlock <= 0 {
		return
	}
	checkSetMax(bounds.NativeAssetPrivacyBodyCeiling, &bounds.MaxNativeAssetPrivacyBodyBytes)
	for _, suite := range p.NativeAssetPrivacySuites {
		if suite.MaxBundleBytes != 0 {
			maximum := min(uint64(suite.MaxBundleBytes)+137,
				uint64(bounds.NativeAssetPrivacyProofCeiling), uint64(p.MaxTxnBytesPerBlock))
			checkSetMax(int(maximum), &bounds.MaxNativeAssetPrivacyProofBytes)
		}
	}
}
