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

package keyauth

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

const namespace = "aprv:authkeys:"
const indexSize = 214

var (
	ErrDomain  = errors.New("invalid privacy key authorization domain")
	ErrRecord  = errors.New("invalid privacy key authorization index record")
	ErrHistory = errors.New("inconsistent privacy key authorization history")
)

// Root separates current key authorization roots from note/history root roles.
// Digest must be unwrapped explicitly at the proof statement boundary.
type Root struct{ Digest tree.Hash }

// Store authenticates current public key records; callers authorize registrations.
// KV must serialize access and atomically apply each batch with rollback on error.
type Store struct {
	kv     tree.KV
	domain tree.Domain
}

type indexRecord struct {
	domain keys.Domain
	index  uint32
	leaf   tree.Hash
	epoch  uint64
}
