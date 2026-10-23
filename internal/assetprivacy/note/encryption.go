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

package note

import (
	"crypto/mlkem"

	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
)

// Encrypt validates context binding and encrypts a canonical opening locally.
func (n Note) Encrypt(publicKey []byte, context envelope.Context) ([]byte, error) {
	if err := n.checkContext(context); err != nil {
		return nil, err
	}
	encoded, err := n.Encode()
	if err != nil {
		return nil, err
	}
	defer clear(encoded)
	return envelope.Seal(publicKey, context, encoded)
}

// Decrypt authenticates, decodes and checks the advertised note commitment.
// Block confirmation and proof validation remain the caller's responsibility.
func Decrypt(key *mlkem.DecapsulationKey768, context envelope.Context, ciphertext []byte) (Note, error) {
	encoded, err := envelope.Open(key, context, ciphertext)
	if err != nil {
		return Note{}, err
	}
	defer clear(encoded)
	n, err := Decode(encoded)
	if err != nil {
		return Note{}, err
	}
	if err = n.checkContext(context); err != nil {
		return Note{}, err
	}
	return n, nil
}

func (n Note) checkContext(context envelope.Context) error {
	if n.Asset != context.Asset || n.Policy != context.Policy {
		return envelope.ErrContext
	}
	commitment, err := n.Commitment(context.Network)
	if err != nil {
		return err
	}
	if commitment != context.Commitment {
		return envelope.ErrContext
	}
	return nil
}
