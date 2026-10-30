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

package proofbridge

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
)

const requestHeader = 1496
const maxRequestBytes = 15664

func (v *Verifier) request(plan *proofbundle.Plan, context Context) ([]byte, error) {
	if plan == nil {
		return nil, proofbundle.ErrPlan
	}
	slots := plan.Slots()
	size, err := v.requestSize(slots)
	if err != nil {
		return nil, err
	}
	encoded := make([]byte, size)
	v.writeHeader(encoded, context, byte(len(slots)))
	at := requestHeader
	for _, slot := range slots {
		at = writeSlot(encoded, at, slot)
	}
	return encoded, nil
}

func (v *Verifier) requestSize(slots []proofbundle.Slot) (int, error) {
	if len(slots) < 1 || len(slots) > proofbundle.MaxSlots {
		return 0, proofbundle.ErrPlan
	}
	size := requestHeader
	for _, slot := range slots {
		entry, exists := v.registry.Entry(artifactregistry.Kind(slot.Kind()))
		count := len(slot.Values())
		if !exists || count != int(entry.PublicCount) {
			return 0, proofbundle.ErrPlan
		}
		size += 8 + 2*count
	}
	if size > maxRequestBytes {
		return 0, ErrLimits
	}
	return size, nil
}
