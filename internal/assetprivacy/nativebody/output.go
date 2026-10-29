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

func outputSize(flags policy.Disclosures) (int, error) {
	if flags & ^(policy.AuditorDisclosure|policy.ControllerDisclosure) != 0 {
		return 0, ErrDisclosures
	}
	count := 1
	if flags&policy.AuditorDisclosure != 0 {
		count++
	}
	if flags&policy.ControllerDisclosure != 0 {
		count++
	}
	return outputHeaderSize + count*envelope.EncodedSize, nil
}

func (output Output) validate() error {
	if output.Nonce == [32]byte{} || output.Commitment == [64]byte{} {
		return ErrOutput
	}
	if _, err := outputSize(output.Disclosures); err != nil {
		return err
	}
	if envelope.CheckEncoding(output.Recipient) != nil {
		return ErrOutput
	}
	if err := checkRole(output.Disclosures&policy.AuditorDisclosure != 0, output.Auditor); err != nil {
		return err
	}
	return checkRole(output.Disclosures&policy.ControllerDisclosure != 0, output.Controller)
}

func checkRole(required bool, encoded []byte) error {
	if !required {
		if len(encoded) != 0 {
			return ErrDisclosures
		}
		return nil
	}
	if envelope.CheckEncoding(encoded) != nil {
		return ErrOutput
	}
	return nil
}

func validateOutputs(outputs []Output) error {
	for _, output := range outputs {
		if err := output.validate(); err != nil {
			return err
		}
	}
	if len(outputs) == 2 && outputs[0].Commitment == outputs[1].Commitment {
		return ErrOutput
	}
	return nil
}
