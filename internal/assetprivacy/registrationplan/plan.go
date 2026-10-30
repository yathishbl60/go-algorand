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

// Package registrationplan derives expected recipient owner-possession statements.
package registrationplan

import (
	"encoding/binary"
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var ErrStatement = errors.New("invalid native owner registration statement")

type Plan struct {
	domain tree.Domain
	owner  [64]byte
	group  [64]byte
}

func New(domain tree.Domain, group, owner [64]byte) (*Plan, error) {
	if domain.Genesis == [32]byte{} || domain.Asset == 0 || domain.Policy == [64]byte{} || group == [64]byte{} || owner == [64]byte{} {
		return nil, ErrStatement
	}
	return &Plan{domain: domain, owner: owner, group: group}, nil
}

func (p *Plan) Domain() tree.Domain { return p.domain }
func (p *Plan) Group() [64]byte     { return p.group }
func (p *Plan) Owner() [64]byte     { return p.owner }
func (p *Plan) Values() []uint16 {
	values := make([]uint16, 64)
	for i := 0; i < 32; i++ {
		values[i] = binary.LittleEndian.Uint16(p.owner[2*i : 2*i+2])
		values[32+i] = binary.LittleEndian.Uint16(p.group[2*i : 2*i+2])
	}
	return values
}
