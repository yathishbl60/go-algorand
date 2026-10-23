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

func canonicalPublicCoefficients(encoded []byte) bool {
	for index := 0; index+2 < len(encoded); index += 3 {
		first, second := decodeCandidatePair(encoded[index], encoded[index+1], encoded[index+2])
		if first >= 3329 || second >= 3329 {
			return false
		}
	}
	return len(encoded) == encodedPublicCoefficients
}

func acceptedMatrixCandidates(prefix []byte) int {
	accepted := 0
	for index := 0; index+2 < len(prefix); index += 3 {
		first, second := decodeCandidatePair(prefix[index], prefix[index+1], prefix[index+2])
		if first < 3329 {
			accepted++
		}
		if second < 3329 {
			accepted++
		}
	}
	return accepted
}

func decodeCandidatePair(low, middle, high byte) (uint16, uint16) {
	return uint16(low) + (uint16(middle&15) << 8), uint16(middle>>4) + (uint16(high) << 4)
}
