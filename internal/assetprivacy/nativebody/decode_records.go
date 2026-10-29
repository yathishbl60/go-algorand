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

func (p *parsed) readRecords(encoded []byte) error {
	at := HeaderSize + p.inputCount*inputSize
	if at > len(encoded) {
		return ErrEncoding
	}
	for index := 0; index < p.inputCount; index++ {
		start := HeaderSize + index*inputSize
		p.inputs[index] = readInput(encoded[start : start+inputSize])
	}
	if err := validateInputs(p.inputs[:p.inputCount]); err != nil {
		return err
	}
	return p.readOutputs(encoded, at)
}

func (p *parsed) readOutputs(encoded []byte, at int) error {
	for index := 0; index < p.outputCount; index++ {
		output, size, err := readOutput(encoded[at:])
		if err != nil {
			return err
		}
		p.outputs[index] = output
		at += size
	}
	if at != len(encoded) {
		return ErrEncoding
	}
	return validateOutputs(p.outputs[:p.outputCount])
}
