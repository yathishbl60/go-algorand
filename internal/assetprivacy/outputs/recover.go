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

package outputs

import (
	"crypto/mlkem"

	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/note"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// Context reconstructs role-bound associated data using a privately supplied epoch.
// Epochs and recipient routing data are never serialized in the public stream.
func (r Record) Context(role envelope.Role, epoch uint64) (envelope.Context, error) {
	if err := r.Validate(); err != nil {
		return envelope.Context{}, err
	}
	if epoch == 0 {
		return envelope.Context{}, envelope.ErrContext
	}
	if _, err := r.ciphertext(role); err != nil {
		return envelope.Context{}, err
	}
	return envelope.Context{Network: r.Domain.Genesis, Asset: r.Domain.Asset, Policy: r.Domain.Policy,
		Epoch: epoch, Role: role, Nonce: r.Nonce, Output: r.Output, Commitment: r.Commitment}, nil
}

// Recover decrypts locally and checks the canonical opening and public commitment.
// Confirmation, membership, ownership, spent state and proof checks remain separate.
func (r Record) Recover(key *mlkem.DecapsulationKey768, role envelope.Role, epoch uint64) (note.Note, error) {
	context, err := r.Context(role, epoch)
	if err != nil {
		return note.Note{}, err
	}
	ciphertext, err := r.ciphertext(role)
	if err != nil {
		return note.Note{}, err
	}
	return note.Decrypt(key, context, ciphertext)
}

func (r Record) ciphertext(role envelope.Role) ([]byte, error) {
	switch role {
	case envelope.Recipient:
		return r.Recipient, nil
	case envelope.Auditor:
		if r.Disclosures&policy.AuditorDisclosure != 0 {
			return r.Auditor, nil
		}
	case envelope.Controller:
		if r.Disclosures&policy.ControllerDisclosure != 0 {
			return r.Controller, nil
		}
	}
	return nil, ErrDisclosures
}
