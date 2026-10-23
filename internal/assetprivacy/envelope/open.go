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

package envelope

import "crypto/mlkem"

// Open authenticates the complete envelope and context before returning plaintext.
// Wrong keys, altered ciphertexts and authentication failures share one error.
func Open(key *mlkem.DecapsulationKey768, context Context, encoded []byte) ([]byte, error) {
	if key == nil {
		return nil, ErrKey
	}
	if len(encoded) != EncodedSize || encoded[0] != formatVersion {
		return nil, ErrCiphertext
	}
	ctx, err := context.Bytes()
	if err != nil {
		return nil, err
	}
	ciphertext := encoded[1 : 1+mlkem.CiphertextSize768]
	aad := associatedData(ctx, key.EncapsulationKey().Bytes(), ciphertext)
	return open(key, ciphertext, encoded[1+mlkem.CiphertextSize768:], aad)
}

func open(key *mlkem.DecapsulationKey768, ciphertext, sealed, aad []byte) ([]byte, error) {
	shared, err := key.Decapsulate(ciphertext)
	if err != nil {
		return nil, ErrCiphertext
	}
	defer clear(shared)
	aead, err := newAEAD(shared, aad)
	if err != nil {
		return nil, ErrCiphertext
	}
	payload, err := aead.Open(nil, nil, sealed, aad)
	if err != nil {
		return nil, ErrCiphertext
	}
	return payload, nil
}
