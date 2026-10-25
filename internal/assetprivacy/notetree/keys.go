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

package notetree

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func prefix(domain tree.Domain) string {
	return "aprv:nt:" + string(binary.BigEndian.AppendUint64(nil, domain.Asset))
}

func markerKey(domain tree.Domain) string { return prefix(domain) + "m" }
func commitmentKey(domain tree.Domain, commitment tree.Hash) string {
	return prefix(domain) + "c" + string(commitment[:])
}

func domainBytes(domain tree.Domain) []byte {
	encoded := append([]byte(nil), domain.Genesis[:]...)
	encoded = binary.BigEndian.AppendUint64(encoded, domain.Asset)
	return append(encoded, domain.Policy[:]...)
}
