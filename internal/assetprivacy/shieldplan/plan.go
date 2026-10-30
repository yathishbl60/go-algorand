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

// Package shieldplan derives the zero-input shield statement, not acceptance.
package shieldplan

import (
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

const PublicValues = 100

var ErrConfig = errors.New("invalid native privacy shield domain")
var ErrStatement = errors.New("invalid native privacy shield statement")
var ErrOutputs = errors.New("native privacy outputs do not match shield statement")

// Plan owns its fixed public fields. It grants no debit or output permission.
type Plan struct {
	domain  tree.Domain
	group   [64]byte
	debit   uint64
	outputs [2][64]byte
	values  [PublicValues]uint16
}

func (p *Plan) Domain() tree.Domain { return p.domain }
func (p *Plan) Group() [64]byte     { return p.group }
func (p *Plan) Debit() uint64       { return p.debit }
func (p *Plan) Outputs() [][64]byte { return append([][64]byte(nil), p.outputs[:]...) }
func (p *Plan) Values() []uint16    { return append([]uint16(nil), p.values[:]...) }

// New binds a positive public debit to exactly two distinct output commitments.
// Ledger authorization, current recipients, proofs and funding remain required.
func New(domain tree.Domain, group [64]byte, debit uint64, outputs [2][64]byte) (*Plan, error) {
	if err := validate(domain, group, debit, outputs); err != nil {
		return nil, err
	}
	return &Plan{domain: domain, group: group, debit: debit, outputs: outputs,
		values: publicValues(group, debit, outputs)}, nil
}
