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

package registrationplan

import (
	"encoding/binary"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// ViewingPlan owns exact ML-KEM public bytes and the finalized transaction group.
type ViewingPlan struct {
	domain    tree.Domain
	group     [64]byte
	publicKey [1184]byte
}

func NewViewing(domain tree.Domain, group [64]byte, publicKey [1184]byte) (*ViewingPlan, error) {
	if domain.Genesis == [32]byte{} || domain.Asset == 0 || domain.Policy == [64]byte{} || group == [64]byte{} {
		return nil, ErrStatement
	}
	if _, err := envelope.ParsePublicKey(publicKey[:]); err != nil {
		return nil, err
	}
	return &ViewingPlan{domain: domain, group: group, publicKey: publicKey}, nil
}

func (p *ViewingPlan) Domain() tree.Domain { return p.domain }
func (p *ViewingPlan) Group() [64]byte     { return p.group }
func (p *ViewingPlan) Values() []uint16 {
	values := make([]uint16, 1216)
	for i, value := range p.publicKey {
		values[i] = uint16(value)
	}
	for i := 0; i < 32; i++ {
		values[1184+i] = binary.LittleEndian.Uint16(p.group[2*i : 2*i+2])
	}
	return values
}
