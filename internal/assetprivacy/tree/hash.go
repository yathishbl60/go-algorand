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

package tree

import (
	"crypto/sha3"
	"encoding/binary"
	"hash"
)

func (d Domain) validate() error {
	if d.Asset == 0 || d.Policy == (Hash{}) || d.Genesis == ([32]byte{}) {
		return ErrDomain
	}
	return nil
}

func (d Domain) hasher(role string) hash.Hash {
	h := sha3.New512()
	h.Write([]byte("Algorand/AssetPrivacy/Candidate1/" + role + "/"))
	h.Write(d.Genesis[:])
	var asset [8]byte
	binary.BigEndian.PutUint64(asset[:], d.Asset)
	h.Write(asset[:])
	h.Write(d.Policy[:])
	return h
}

// NodeHash uses level zero immediately above the raw commitment leaves.
func (d Domain) NodeHash(level byte, left, right Hash) Hash {
	h := d.hasher("MerkleNode")
	h.Write([]byte{level})
	h.Write(left[:])
	h.Write(right[:])
	var out Hash
	h.Sum(out[:0])
	return out
}

func (d Domain) emptyNodes() [Depth + 1]Hash {
	var nodes [Depth + 1]Hash
	d.hasher("MerkleEmpty").Sum(nodes[0][:0])
	for level := byte(0); level < Depth; level++ {
		nodes[level+1] = d.NodeHash(level, nodes[level], nodes[level])
	}
	return nodes
}
