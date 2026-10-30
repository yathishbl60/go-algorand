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

// Package proofbundle frames the exact expected native proof set, not authorization.
package proofbundle

import "errors"

var (
	ErrPlan   = errors.New("invalid native privacy proof bundle plan")
	ErrLimits = errors.New("invalid native privacy proof bundle limits")
	ErrFrame  = errors.New("native privacy proof bundle does not match expected slots")
)

const (
	HeaderSize        = 7
	EntrySize         = 10
	MaxSlots          = 7
	MainOutput uint32 = 0xffffffff
)

const magic = "NAPB"
const version byte = 1
const starkMagic = "NAPSTK\x01\x00"
const starkHeader = 73

type Kind byte

const (
	Transfer          Kind = 1
	Unshield          Kind = 2
	ShieldKind        Kind = 3
	PartialPrivate    Kind = 4
	FullPrivate       Kind = 5
	PartialPublic     Kind = 6
	FullPublic        Kind = 7
	Recipient         Kind = 17
	Disclosure        Kind = 18
	RetainedRecipient Kind = 19
	RecoveryRecipient Kind = 20
)

// Slot is an immutable expected descriptor and public vector, selected by a plan.
type Slot struct {
	kind   Kind
	output uint32
	role   byte
	values []uint16
}

func (s Slot) Kind() Kind       { return s.kind }
func (s Slot) Output() uint32   { return s.output }
func (s Slot) Role() byte       { return s.role }
func (s Slot) Values() []uint16 { return append([]uint16(nil), s.values...) }

type Plan struct{ slots []Slot }

func (p *Plan) Slots() []Slot { return append([]Slot(nil), p.slots...) }
