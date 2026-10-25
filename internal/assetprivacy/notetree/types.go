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

// Package notetree enforces one creation of each native note commitment.
package notetree

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var (
	ErrDuplicate  = errors.New("native privacy note commitment already exists")
	ErrGeneration = errors.New("unsupported native privacy note tree generation")
	ErrEncoding   = errors.New("invalid native privacy note commitment index")
)

// Store is an append-only generation2 snapshot with no exposed replacement capability.
// KV access must be serialized and every Apply atomic with rollback on failure.
type Store struct {
	kv     tree.KV
	domain tree.Domain
	root   tree.Hash
	count  uint64
}

func (s *Store) Root() tree.Hash { return s.root }
func (s *Store) Count() uint64   { return s.count }
