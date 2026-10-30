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

package artifactregistry

import "crypto/sha3"

const digestDomain = "Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV4/"

// Digest is a bounded integrity helper, not validation, provenance or approval.
// Only Load checks the independent pins and complete native layout contract.
func Digest(encoded []byte) ([64]byte, error) {
	if len(encoded) != EncodedSize {
		return [64]byte{}, ErrEncoding
	}
	return sum(encoded), nil
}

func sum(encoded []byte) [64]byte {
	var message [len(digestDomain) + EncodedSize]byte
	copy(message[:], digestDomain)
	copy(message[len(digestDomain):], encoded)
	return sha3.Sum512(message[:])
}
