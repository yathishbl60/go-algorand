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

// GenerateKey uses the operating system RNG; retain the seed in encrypted storage.
func GenerateKey() (*mlkem.DecapsulationKey768, error) {
	return mlkem.GenerateKey768()
}

// RestoreKey derives a decapsulation key from its canonical 64-byte secret seed.
func RestoreKey(seed []byte) (*mlkem.DecapsulationKey768, error) {
	key, err := mlkem.NewDecapsulationKey768(seed)
	if err != nil {
		return nil, ErrKey
	}
	return key, nil
}

// ParsePublicKey enforces ML-KEM's length and modulus checks before encapsulation.
func ParsePublicKey(encoded []byte) (*mlkem.EncapsulationKey768, error) {
	key, err := mlkem.NewEncapsulationKey768(encoded)
	if err != nil {
		return nil, ErrKey
	}
	return key, nil
}
