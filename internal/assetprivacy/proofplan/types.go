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

// Package proofplan derives expected public inputs; it does not verify proofs.
package proofplan

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var (
	ErrConfig = errors.New("invalid native privacy proof plan configuration")
	ErrOutput = errors.New("native privacy output does not match proof plan state")
	ErrRoles  = errors.New("native privacy proof roles do not exactly match policy")
)

// Config must come from current group-local state, never proof-advertised values.
type Config struct {
	Domain          tree.Domain
	Disclosures     policy.Disclosures
	KeyRoot         keyauth.Root
	EligibilityRoot eligibility.Root
	Auditor         [32]byte
	Controller      [32]byte
}

type Kind byte

const (
	RecipientProof Kind = 1 + iota
	DisclosureProof
	RetainedRecipientProof
	RecoveryRecipientProof
)

const (
	RecipientPublicValues         = 1132
	DisclosurePublicValues        = 1132
	RetainedRecipientPublicValues = 1100
	RecoveryRecipientPublicValues = 1100
)

// Slot holds an immutable expected statement for one role of one output.
type Slot struct {
	role   envelope.Role
	kind   Kind
	values []uint16
}

func (s Slot) Role() envelope.Role { return s.role }
func (s Slot) Kind() Kind          { return s.kind }
func (s Slot) Values() []uint16    { return append([]uint16(nil), s.values...) }

// Plan requires one recipient proof and exactly the policy's disclosure roles.
// It grants no authority to append outputs, spend notes or accept transactions.
type Plan struct {
	slots []Slot
}

func (p *Plan) Slots() []Slot { return append([]Slot(nil), p.slots...) }
