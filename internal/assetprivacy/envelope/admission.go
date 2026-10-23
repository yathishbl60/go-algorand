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

package envelope

import (
	"crypto/mlkem"
	"crypto/sha3"
)

// MatrixPrefixBlocks fixes the candidate admission bound; proofs cannot select it.
const MatrixPrefixBlocks = 16

const encodedPublicCoefficients = 1152

// CheckPublicKey checks candidate admission without constructing an ML-KEM key.
// Matrix-prefix exhaustion is rejected even for otherwise standard-valid keys.
func CheckPublicKey(encoded []byte) error {
	if len(encoded) != mlkem.EncapsulationKeySize768 {
		return ErrKey
	}
	if !canonicalPublicCoefficients(encoded[:encodedPublicCoefficients]) {
		return ErrKey
	}
	if !admittedMatrixSeed([32]byte(encoded[encodedPublicCoefficients:])) {
		return ErrKey
	}
	return nil
}

func admittedMatrixSeed(seed [32]byte) bool {
	admitted := true
	for first := byte(0); first < 3; first++ {
		for second := byte(0); second < 3; second++ {
			if !matrixPrefixFits(seed, first, second) {
				admitted = false
			}
		}
	}
	return admitted
}

func matrixPrefixFits(seed [32]byte, first, second byte) bool {
	var input [34]byte
	copy(input[:32], seed[:])
	input[32], input[33] = first, second
	prefix := sha3.SumSHAKE128(input[:], MatrixPrefixBlocks*168)
	return acceptedMatrixCandidates(prefix) >= 256
}
