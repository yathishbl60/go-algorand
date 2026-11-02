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

package suitecatalog

import (
	"encoding/binary"
	"testing"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
)

// Fixed format fixtures contain a synthetic root; they cannot authenticate a proof.
var scopeLayouts = [20][5]uint32{
	{1, 16384, 8506, 80, 264}, {2, 16384, 8506, 80, 196},
	{3, 1024, 7641, 8, 100}, {4, 8192, 8539, 60, 192},
	{5, 8192, 8539, 60, 160}, {6, 8192, 8539, 60, 164},
	{7, 8192, 8539, 60, 132}, {8, 16384, 8506, 80, 392},
	{9, 16384, 8506, 80, 324}, {10, 1024, 7641, 8, 228},
	{11, 8192, 8539, 60, 320}, {12, 8192, 8539, 60, 288},
	{13, 8192, 8539, 60, 292}, {14, 8192, 8539, 60, 260},
	{17, 65536, 24869, 2075, 1132}, {18, 65536, 23973, 2067, 1132},
	{19, 65536, 24229, 2067, 1100}, {20, 65536, 24869, 2071, 1100},
	{21, 512, 1953, 11, 64}, {22, 16384, 9394, 723, 1216},
}

func scopeFixture(t *testing.T, kinds uint32) ([]byte, config.NativeAssetPrivacySuite) {
	t.Helper()
	var profile, reference [64]byte
	profile[0], reference[0] = 1, 2
	encoded := scopeBytes(profile, reference)
	digest, err := artifactregistry.Digest(encoded)
	if err != nil {
		t.Fatal(err)
	}
	return encoded, config.NativeAssetPrivacySuite{ID: profile, Reference: reference,
		RegistryDigest: digest, ProofKinds: kinds, MaxBundleBytes: 1 << 20,
		MaxProofBytes: 1 << 20, MaxAllocationBytes: 1 << 20}
}

func scopeBytes(profile, reference [64]byte) []byte {
	encoded := make([]byte, artifactregistry.EncodedSize)
	copy(encoded[:5], "NAPK\x07")
	copy(encoded[5:69], profile[:])
	copy(encoded[69:133], reference[:])
	encoded[133] = 20
	for index, layout := range scopeLayouts {
		slot := encoded[134+82*index : 134+82*(index+1)]
		slot[0] = byte(layout[0])
		for column, value := range layout[1:] {
			binary.LittleEndian.PutUint32(slot[1+4*column:5+4*column], value)
		}
		if layout[0] == uint32(artifactregistry.OwnerRegistration) {
			slot[17], slot[18] = 3, 1
		}
	}
	return encoded
}

func scopeSelection(t *testing.T, record config.NativeAssetPrivacySuite) Suite {
	t.Helper()
	suite, err := Lookup(config.ConsensusParams{EnableNativeAssetPrivacy: true,
		NativeAssetPrivacySuites: []config.NativeAssetPrivacySuite{record}}, record.ID)
	if err != nil {
		t.Fatal(err)
	}
	return suite
}
