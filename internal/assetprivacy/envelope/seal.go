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

// Seal encrypts one fixed-size payload locally with fresh KEM and GCM randomness.
// The public key is bound to the encryption but is not included in the envelope.
func Seal(publicKey []byte, context Context, payload []byte) ([]byte, error) {
	if len(payload) != PayloadSize {
		return nil, ErrPayload
	}
	encoded, err := context.Bytes()
	if err != nil {
		return nil, err
	}
	key, err := ParsePublicKey(publicKey)
	if err != nil {
		return nil, err
	}
	shared, ciphertext := key.Encapsulate()
	defer clear(shared)
	return seal(shared, ciphertext, associatedData(encoded, publicKey, ciphertext), payload)
}

func seal(shared, ciphertext, aad, payload []byte) ([]byte, error) {
	aead, err := newAEAD(shared, aad)
	if err != nil {
		return nil, err
	}
	out := make([]byte, 1, EncodedSize)
	out[0] = formatVersion
	out = append(out, ciphertext...)
	return aead.Seal(out, nil, payload, aad), nil
}
