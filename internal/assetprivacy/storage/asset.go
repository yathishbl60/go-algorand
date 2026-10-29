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

import (
	"encoding/binary"
	"strings"
)

// Asset identifies only known canonical native namespaces. Counter keys are
// deliberately excluded: the ledger must update those through its private meter.
func Asset(key string) (uint64, bool, error) {
	if !strings.HasPrefix(key, "aprv:") {
		return 0, false, nil
	}
	for _, wrapper := range []string{"aprv:authkeys:", "aprv:eligibility:"} {
		if tail, ok := strings.CutPrefix(key, wrapper); ok {
			return wrappedAsset(wrapper, tail)
		}
	}
	return directAsset(key)
}

func wrappedAsset(wrapper, tail string) (uint64, bool, error) {
	if strings.HasPrefix(tail, "aprv:t:") {
		return directAsset(tail)
	}
	length := 137 // 'i' plus eligibility's 136-byte domain.
	if wrapper == "aprv:authkeys:" {
		length++ // Key domain also includes its role byte.
	}
	if len(tail) != length || tail[0] != 'i' {
		return 0, true, ErrNamespace
	}
	return assetNumber(tail[33:41])
}

func directAsset(key string) (uint64, bool, error) {
	kind, tail, ok := strings.Cut(key[5:], ":")
	if !ok {
		return 0, true, ErrNamespace
	}
	if kind == "k" || kind == "f" {
		return domainAsset(kind, tail)
	}
	if len(tail) < 8 || !validSuffix(kind, tail[8:]) {
		return 0, true, ErrNamespace
	}
	return assetNumber(tail[:8])
}

func assetNumber(encoded string) (uint64, bool, error) {
	asset := binary.BigEndian.Uint64([]byte(encoded))
	if asset == 0 {
		return 0, true, ErrNamespace
	}
	return asset, true, nil
}
