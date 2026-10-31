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

package shieldplan

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

const ConfidentialPublicValues = 228

func NewConfidential(domain tree.Domain, group [64]byte, debit uint64, outputs [2][64]byte, participants [4][32]byte) (*Plan, error) {
	if participants[0] != [32]byte{} || participants[1] != [32]byte{} ||
		participants[2] == [32]byte{} || participants[3] == [32]byte{} {
		return nil, ErrStatement
	}
	plan, err := New(domain, group, debit, outputs)
	if err != nil {
		return nil, err
	}
	plan.confidential, plan.participants = true, participants
	return plan, nil
}

func (p *Plan) Confidential() bool        { return p.confidential }
func (p *Plan) Participants() [4][32]byte { return p.participants }

func (p *Plan) Values() []uint16 {
	values := append([]uint16(nil), p.values[:]...)
	if !p.confidential {
		return values
	}
	for _, account := range p.participants {
		for _, value := range account {
			values = append(values, uint16(value))
		}
	}
	return values
}
