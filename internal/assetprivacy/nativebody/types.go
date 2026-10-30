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

// Package nativebody implements the bounded canonical candidate aprv body format.
// Parsing and validation grant no authority and perform no ledger or proof checks.
package nativebody

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

const HeaderSize = 225
const MaxSize = 10465
const inputSize = 136
const outputHeaderSize = 97

type Kind byte

const (
	Transfer       Kind = 1
	Unshield       Kind = 2
	Shield         Kind = 3
	PartialPrivate Kind = 4
	FullPrivate    Kind = 5
	PartialPublic  Kind = 6
	FullPublic     Kind = 7
)

// Body contains public wire fields only; private witnesses belong outside the body.
type Body struct {
	Operation      Kind
	Mode           policy.Modes
	Asset          uint64
	Policy         [64]byte
	Suite          [64]byte
	Debit          uint64
	DebitSource    [32]byte
	Credit         uint64
	CreditReceiver [32]byte
	Inputs         []Input
	Outputs        []Output
}

type Input struct {
	Nullifier [64]byte
	Root      [64]byte
	Count     uint64
}

type Output struct {
	Nonce       [32]byte
	Commitment  [64]byte
	Disclosures policy.Disclosures
	Recipient   []byte
	Auditor     []byte
	Controller  []byte
}
