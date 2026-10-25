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

package proofbundle

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (p *Plan) addOutputs(domain tree.Domain, commitments [][64]byte, outputs []*proofplan.Plan, remainder int, recipient Kind) error {
	if len(commitments) != len(outputs) || len(outputs) > 2 {
		return ErrPlan
	}
	for index, output := range outputs {
		if output == nil || output.Domain() != domain || output.Ordinal() != uint32(index) || output.Commitment() != commitments[index] {
			return ErrPlan
		}
		kind := recipient
		if index == remainder {
			kind = RetainedRecipient
		}
		if err := p.addOutput(output, kind); err != nil {
			return err
		}
	}
	return p.validate()
}

func (p *Plan) addOutput(output *proofplan.Plan, recipient Kind) error {
	slots := output.Slots()
	if len(slots) < 1 || len(slots) > 3 {
		return ErrPlan
	}
	previous := byte(0)
	for index, slot := range slots {
		kind, role := Kind(slot.Kind())+16, byte(slot.Role())
		if !validOutputSlot(index, previous, kind, role, recipient) {
			return ErrPlan
		}
		p.slots = append(p.slots, Slot{kind: kind, output: output.Ordinal(), role: role, values: slot.Values()})
		previous = role
	}
	return nil
}

func validOutputSlot(index int, previous byte, kind Kind, role byte, recipient Kind) bool {
	if index == 0 {
		return kind == recipient && role == 1
	}
	return kind == Disclosure && role > previous && role >= 2 && role <= 3
}
