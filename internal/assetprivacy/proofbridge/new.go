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

package proofbridge

import "github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"

// NewVerifier validates and copies the exact registry; caller pins must be trusted
// independently of the file/proof. Caller must keep registryBytes stable during copy.
func NewVerifier(registryBytes []byte, pin artifactregistry.Pin, limits Limits) (*Verifier, error) {
	if err := limits.validate(); err != nil {
		return nil, err
	}
	if len(registryBytes) != artifactregistry.EncodedSize {
		return nil, artifactregistry.ErrEncoding
	}
	var encoded [artifactregistry.EncodedSize]byte
	copy(encoded[:], registryBytes)
	registry, err := artifactregistry.Load(encoded[:], pin)
	if err != nil {
		return nil, err
	}
	if err = backendStatus(pin.Implementation); err != nil {
		return nil, err
	}
	return &Verifier{registry: registry, encoded: encoded, pin: pin, limits: limits}, nil
}
