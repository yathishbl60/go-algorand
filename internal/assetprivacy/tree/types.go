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

// Depth bounds the append-only tree to 2^32 commitments.
const Depth = 32
const Capacity uint64 = 1 << Depth

// Hash is a raw 64-byte commitment or SHA3-512 node digest.
type Hash = [64]byte

// Domain binds every hash to its network, asset and immutable policy.
type Domain struct {
	Genesis [32]byte
	Asset   uint64
	Policy  [64]byte
}

// Record is a canonical native KV write.
type Record struct {
	Key   string
	Value []byte
}

// KV must serialize operations and apply each batch atomically, or roll it back.
// Tree never mutates buffers returned by Get; implementations must copy writes.
type KV interface {
	Get(string) ([]byte, bool, error)
	Apply([]Record) error
}

// Witness carries bottom-up siblings; bit level of Index selects right when set.
type Witness struct {
	Index    uint32
	Leaf     Hash
	Siblings [Depth]Hash
	Root     Hash
	Count    uint64
}

// Tree is a snapshot; callers must serialize KV access and reopen after others append.
type Tree struct {
	kv     KV
	domain Domain
	empty  [Depth + 1]Hash
	root   Hash
	count  uint64
}

func (t *Tree) Root() Hash    { return t.root }
func (t *Tree) Count() uint64 { return t.count }
