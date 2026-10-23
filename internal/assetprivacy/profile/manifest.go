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

// Package profile checks local cryptographic profile evidence bundles.
// It does not approve cryptography or enable any consensus feature.
package profile

const (
	manifestVersion  = 1
	maxManifestBytes = 64 << 10
	maxArtifactBytes = 256 << 20
)

// Manifest describes evidence for review, not an on-chain cryptographic suite.
type Manifest struct {
	Version   uint32     `json:"version"`
	ProfileID string     `json:"profile_id"`
	Artifacts []Artifact `json:"artifacts"`
}

// Artifact identifies one required document or archive by its SHA-384 digest.
type Artifact struct {
	Role   string `json:"role"`
	Path   string `json:"path"`
	SHA384 string `json:"sha384"`
}

// Report records successful file integrity checks, not security approval.
type Report struct {
	ProfileID         string `json:"profile_id"`
	VerifiedArtifacts int    `json:"verified_artifacts"`
	Scope             string `json:"scope"`
}
