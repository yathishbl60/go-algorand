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

import (
	"maps"
	"slices"
)

// NativeAssetPrivacySuite pins a verification profile, implementation reference,
// registry digest and trusted work caps. Metadata alone never enables privacy.
// All current consensus versions leave the allowed suite list empty.
type NativeAssetPrivacySuite struct {
	ID                 [64]byte
	Reference          [64]byte
	RegistryDigest     [64]byte
	MaxBundleBytes     uint32
	MaxProofBytes      uint32
	MaxAllocationBytes uint32
}

func cloneConsensusParams(params ConsensusParams) ConsensusParams {
	params.ApprovedUpgrades = maps.Clone(params.ApprovedUpgrades)
	params.NativeAssetPrivacySuites = slices.Clone(params.NativeAssetPrivacySuites)
	return params
}
