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

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func readOutput(encoded []byte) (Output, int, error) {
	if len(encoded) < outputHeaderSize {
		return Output{}, 0, ErrEncoding
	}
	size, err := outputSize(policy.Disclosures(encoded[96]))
	if err != nil {
		return Output{}, 0, err
	}
	if len(encoded) < size {
		return Output{}, 0, ErrEncoding
	}
	output := outputView(encoded[:size])
	return output, size, output.validate()
}

func outputView(encoded []byte) Output {
	output := Output{Disclosures: policy.Disclosures(encoded[96])}
	copy(output.Nonce[:], encoded[:32])
	copy(output.Commitment[:], encoded[32:96])
	output.Recipient = encoded[97 : 97+envelope.EncodedSize]
	at := 97 + envelope.EncodedSize
	if output.Disclosures&policy.AuditorDisclosure != 0 {
		output.Auditor = encoded[at : at+envelope.EncodedSize]
		at += envelope.EncodedSize
	}
	if output.Disclosures&policy.ControllerDisclosure != 0 {
		output.Controller = encoded[at : at+envelope.EncodedSize]
	}
	return output
}

func (output Output) clone() Output {
	output.Recipient = append([]byte(nil), output.Recipient...)
	output.Auditor = append([]byte(nil), output.Auditor...)
	output.Controller = append([]byte(nil), output.Controller...)
	return output
}
