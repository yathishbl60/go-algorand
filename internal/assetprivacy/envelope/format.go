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

// Package envelope implements the candidate local note-encryption format.
// It does not prove encryption correctness or establish recipient anonymity.
package envelope

import (
	"crypto/mlkem"
	"errors"
)

const (
	// PayloadSize fixes ciphertext length independently of note contents.
	PayloadSize = 512
	// EncodedSize includes version, KEM ciphertext, nonce, payload and GCM tag.
	EncodedSize        = 1 + mlkem.CiphertextSize768 + 12 + PayloadSize + 16
	formatVersion byte = 1
)

var (
	ErrContext    = errors.New("invalid private note encryption context")
	ErrPayload    = errors.New("private note payload must be exactly 512 bytes")
	ErrKey        = errors.New("invalid private note encryption key")
	ErrCiphertext = errors.New("private note decryption failed")
)

// Role separates recipient, auditor and controller encryption uses.
type Role byte

const (
	Recipient Role = 1 + iota
	Auditor
	Controller
)
