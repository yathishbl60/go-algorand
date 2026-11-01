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

package libgoal

import (
	"bytes"
	"crypto/sha3"
	"slices"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
)

// NewNativePrivacyScanner retains owned key handles, never uploading them.
// The configured RPC must be trusted for complete and authentic output records.
func NewNativePrivacyScanner(client *apiclient.NativePrivacyClient, domain NativePrivacyScanDomain, keys []NativePrivacyViewingKey) (*NativePrivacyScanner, error) {
	if client == nil || domain.Network == [32]byte{} || domain.Asset == 0 || domain.Policy == [64]byte{} || domain.Disclosures > 3 {
		return nil, ErrNativePrivacyScanConfig
	}
	if len(keys) == 0 || len(keys) > NativePrivacyScanMaxKeys {
		return nil, ErrNativePrivacyScanConfig
	}
	identities, err := nativePrivacyScanIdentities(keys)
	if err != nil {
		return nil, err
	}
	coverage := nativePrivacyScanCoverage(domain, identities)
	return &NativePrivacyScanner{client: client, domain: domain, keys: slices.Clone(keys), coverage: coverage}, nil
}

func nativePrivacyScanIdentities(keys []NativePrivacyViewingKey) ([][64]byte, error) {
	identities := make([][64]byte, len(keys))
	for i, key := range keys {
		if key.key == nil || key.epoch == 0 || key.role < 1 || key.role > 3 {
			return nil, ErrNativePrivacyScanConfig
		}
		identities[i] = key.identity
	}
	slices.SortFunc(identities, func(a, b [64]byte) int { return bytes.Compare(a[:], b[:]) })
	for i := 1; i < len(identities); i++ {
		if identities[i] == identities[i-1] {
			return nil, ErrNativePrivacyScanConfig
		}
	}
	return identities, nil
}

func nativePrivacyScanCoverage(domain NativePrivacyScanDomain, identities [][64]byte) [64]byte {
	hash := sha3.New512()
	hash.Write([]byte("Algorand/AssetPrivacy/ScannerCoverage/V1/"))
	hash.Write(domain.Network[:])
	hash.Write(domain.Policy[:])
	nativePrivacyScanCoverageNumbers(hash, uint64(domain.Asset), domain.Disclosures)
	for _, identity := range identities {
		hash.Write(identity[:])
	}
	var result [64]byte
	hash.Sum(result[:0])
	return result
}
