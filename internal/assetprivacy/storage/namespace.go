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

package storage

func validSuffix(kind, suffix string) bool {
	switch kind {
	case "p", "s", "d":
		return len(suffix) == 0
	case "n":
		return len(suffix) == 64
	case "o":
		return suffix == "m" || (len(suffix) == 5 && suffix[0] == 'e')
	case "t":
		return suffix == "m" || suffix == "h" || taggedSuffix(suffix, 'n', 65) || taggedSuffix(suffix, 'r', 9)
	case "nt":
		return suffix == "m" || taggedSuffix(suffix, 'c', 65)
	}
	return false
}

func taggedSuffix(suffix string, tag byte, length int) bool {
	return len(suffix) == length && suffix[0] == tag
}

func domainAsset(kind, tail string) (uint64, bool, error) {
	length := 136
	if kind == "k" {
		length++
	}
	if len(tail) < length || !validEpochSuffix(tail[length:]) {
		return 0, true, ErrNamespace
	}
	return assetNumber(tail[32:40])
}

func validEpochSuffix(suffix string) bool {
	return suffix == "c" || taggedSuffix(suffix, 'e', 9)
}
