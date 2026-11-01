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

package libgoal

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/note"
)

// ExportOpening explicitly returns owned secret plaintext. The caller must keep
// it private and clear the returned bytes when finished; no RPC needs this data.
func (recovered *NativePrivacyRecoveredNote) ExportOpening() ([]byte, error) {
	if recovered == nil {
		return nil, note.ErrNote
	}
	return recovered.opening.Encode()
}

func (recovered *NativePrivacyRecoveredNote) Value() uint64 {
	if recovered == nil {
		return 0
	}
	return recovered.opening.Value
}

func (recovered *NativePrivacyRecoveredNote) Account() basics.Address {
	if recovered == nil {
		return basics.Address{}
	}
	return basics.Address(recovered.opening.Account)
}

func (recovered *NativePrivacyRecoveredNote) Owner() [64]byte {
	if recovered == nil {
		return [64]byte{}
	}
	return recovered.opening.Owner
}

// Nullifier is a local derived marker, not proof of spendability or freshness.
func (recovered *NativePrivacyRecoveredNote) Nullifier() ([64]byte, error) {
	if recovered == nil {
		return [64]byte{}, note.ErrNote
	}
	return recovered.opening.Nullifier(recovered.network)
}
