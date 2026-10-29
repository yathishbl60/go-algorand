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

// Encode validates and returns owned canonical public body bytes.
// Keep every body slice immutable until this call returns.
func Encode(body Body) ([]byte, error) {
	if err := body.Validate(); err != nil {
		return nil, err
	}
	encoded := make([]byte, encodedSize(body))
	writeHeader(encoded, body)
	at := HeaderSize
	for _, input := range body.Inputs {
		writeInput(encoded[at:at+inputSize], input)
		at += inputSize
	}
	for _, output := range body.Outputs {
		at += writeOutput(encoded[at:], output)
	}
	return encoded, nil
}

func encodedSize(body Body) int {
	size := HeaderSize + len(body.Inputs)*inputSize
	for _, output := range body.Outputs {
		outputBytes, _ := outputSize(output.Disclosures)
		size += outputBytes
	}
	return size
}

func writeOutput(encoded []byte, output Output) int {
	copy(encoded[:32], output.Nonce[:])
	copy(encoded[32:96], output.Commitment[:])
	encoded[96] = byte(output.Disclosures)
	at := outputHeaderSize
	at += copy(encoded[at:], output.Recipient)
	at += copy(encoded[at:], output.Auditor)
	return at + copy(encoded[at:], output.Controller)
}
