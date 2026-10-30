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

// Package artifactregistry reads independently pinned candidate preprocessing roots.
// It does not approve a suite, authenticate proofs or enable native privacy.
package artifactregistry

import "errors"

const EncodedSize = 1036
const EntryCount = 11
const headerSize = 134
const entrySize = 82

var (
	ErrPin      = errors.New("invalid native privacy artifact registry pin")
	ErrEncoding = errors.New("invalid native privacy artifact registry encoding")
	ErrLayout   = errors.New("native privacy artifact registry layout mismatch")
)

// Pin must come from trusted independent suite configuration, never the file itself.
type Pin struct {
	Profile        [64]byte
	Implementation [64]byte
	Digest         [64]byte
}

type Kind byte

const (
	Transfer          Kind = 1
	Unshield          Kind = 2
	Shield            Kind = 3
	PartialPrivate    Kind = 4
	FullPrivate       Kind = 5
	PartialPublic     Kind = 6
	FullPublic        Kind = 7
	Recipient         Kind = 17
	Disclosure        Kind = 18
	RetainedRecipient Kind = 19
	RecoveryRecipient Kind = 20
)

// Entry contains canonical dimensions and eight raw SHA3 root words, not field cells.
type Entry struct {
	Kind        Kind
	Rows        uint32
	MainWidth   uint32
	PreWidth    uint32
	PublicCount uint32
	Flags       byte
	Root        [8]uint64
}

// Registry owns fixed value arrays. Its zero value is not a loaded registry.
type Registry struct {
	pin     Pin
	entries [EntryCount]Entry
}
