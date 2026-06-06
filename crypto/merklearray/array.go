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

package merklearray

import (
	"hash"

	"github.com/algorand/go-algorand/crypto"
)

// An Array represents a dense array of leaf elements that are
// combined into a Merkle tree. The Marshal method returns a byte slice that represents the object
// that the Tree will use to hash the leaves.

// An Array is an interface that is being using when creating Merkle trees.
// It represents a dense array of n (n is given by the Length() method) elements,
// and returns a hash representation for each leaf (in the range)
type Array interface {
	// Length returns number of elements in the array.
	Length() uint64

	// Marshal Returns a hash representation of the element located in position pos
	Marshal(pos uint64) (crypto.Hashable, error)
}

// HashableArray can hash a leaf directly into the supplied hash function
// without materializing an intermediate Hashable wrapper.
type HashableArray interface {
	Hash(pos uint64, h hash.Hash) (crypto.GenericDigest, error)
}

// HashableArrayInto can hash a leaf directly into the provided output buffer.
type HashableArrayInto interface {
	HashInto(pos uint64, h hash.Hash, out []byte) error
}
