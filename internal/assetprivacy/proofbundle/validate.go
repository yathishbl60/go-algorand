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

func (p *Plan) validate() error {
	if p == nil || len(p.slots) < 1 || len(p.slots) > MaxSlots {
		return ErrPlan
	}
	for _, slot := range p.slots {
		if len(slot.values) != publicCount(slot.kind) {
			return ErrPlan
		}
	}
	return nil
}

func publicCount(kind Kind) int {
	counts := [...]int{0, 264, 196, 0, 192, 160, 164, 132}
	if int(kind) < len(counts) && counts[kind] != 0 {
		return counts[kind]
	}
	switch kind {
	case Recipient, Disclosure:
		return 1132
	case RetainedRecipient, RecoveryRecipient:
		return 1100
	default:
		return -1
	}
}
