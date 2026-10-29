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

package nativebody

import "encoding/binary"

func (input Input) validate() error {
	if input.Root == [64]byte{} || input.Count == 0 || input.Count > 1<<32 {
		return ErrInput
	}
	return nil // Every 64-byte nullifier pattern, including zero, is allowed.
}

func validateInputs(inputs []Input) error {
	for _, input := range inputs {
		if err := input.validate(); err != nil {
			return err
		}
	}
	if len(inputs) == 2 && inputs[0].Nullifier == inputs[1].Nullifier {
		return ErrInput
	}
	return nil
}

func readInput(encoded []byte) Input {
	var input Input
	copy(input.Nullifier[:], encoded[:64])
	copy(input.Root[:], encoded[64:128])
	input.Count = binary.BigEndian.Uint64(encoded[128:136])
	return input
}

func writeInput(encoded []byte, input Input) {
	copy(encoded[:64], input.Nullifier[:])
	copy(encoded[64:128], input.Root[:])
	binary.BigEndian.PutUint64(encoded[128:136], input.Count)
}
