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
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
)

// Pin returns consensus-selected values; Implementation is the legacy wire name
// for the compatible implementation reference, not a proof-supplied selection.
func (s Suite) Pin() artifactregistry.Pin {
	return artifactregistry.Pin{Profile: s.record.ID, Implementation: s.record.Reference,
		Digest: s.record.RegistryDigest}
}

func (s Suite) Limits() proofbridge.Limits {
	return proofbridge.Limits{MaxBundleBytes: s.record.MaxBundleBytes,
		MaxProofBytes: s.record.MaxProofBytes, MaxAllocationBytes: s.record.MaxAllocationBytes}
}

// Verifier checks untrusted registry bytes against selected pins and the backend ABI.
// No approval, generated registry or callback can substitute for NewVerifier checks.
func (s Suite) Verifier(registryBytes []byte) (*proofbridge.Verifier, error) {
	if err := checkRecord(s.record); err != nil {
		return nil, err
	}
	return proofbridge.NewVerifier(registryBytes, s.Pin(), s.Limits())
}
