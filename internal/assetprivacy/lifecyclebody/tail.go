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

package lifecyclebody

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

func (b *Body) readTail(encoded []byte) error {
	switch b.Operation {
	case Adopt:
		b.Modes, b.Capabilities, b.Disclosures = policy.Modes(encoded[0]), policy.Capabilities(encoded[1]), policy.Disclosures(encoded[2])
		copy(b.Auditor[:], encoded[3:35])
	case Assign:
		copy(b.Auditor[:], encoded[:32])
	case Freeze:
		copy(b.Target[:], encoded[:32])
		if encoded[32] > 1 {
			return ErrFields
		}
		b.Frozen = encoded[32] == 1
	}
	return nil
}

func writeTail(encoded []byte, body Body) {
	switch body.Operation {
	case Adopt:
		encoded[0], encoded[1], encoded[2] = byte(body.Modes), byte(body.Capabilities), byte(body.Disclosures)
		copy(encoded[3:35], body.Auditor[:])
	case Assign:
		copy(encoded[:32], body.Auditor[:])
	case Freeze:
		copy(encoded[:32], body.Target[:])
		if body.Frozen {
			encoded[32] = 1
		}
	}
}

func (b Body) decodeTail(encoded []byte) (Body, error) {
	if err := b.readTail(encoded); err != nil {
		return Body{}, err
	}
	if err := b.Validate(); err != nil {
		return Body{}, err
	}
	return b, nil
}
