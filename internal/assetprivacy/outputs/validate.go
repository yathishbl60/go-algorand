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
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func checkDisclosures(flags policy.Disclosures) error {
	if flags & ^(policy.AuditorDisclosure|policy.ControllerDisclosure) != 0 {
		return ErrDisclosures
	}
	return nil
}

// Validate checks canonical framing and declared role presence, not encryption proofs.
func (r Record) Validate() error {
	if err := checkDomain(r.Domain); err != nil {
		return err
	}
	if r.Nonce == [32]byte{} || r.Commitment == [64]byte{} {
		return ErrEncoding
	}
	if err := checkDisclosures(r.Disclosures); err != nil {
		return err
	}
	if err := envelope.CheckEncoding(r.Recipient); err != nil {
		return ErrEncoding
	}
	if err := checkOptional(r.Disclosures, policy.AuditorDisclosure, r.Auditor); err != nil {
		return err
	}
	return checkOptional(r.Disclosures, policy.ControllerDisclosure, r.Controller)
}

func checkOptional(flags, role policy.Disclosures, value []byte) error {
	if flags&role == 0 {
		if len(value) != 0 {
			return ErrDisclosures
		}
		return nil
	}
	if envelope.CheckEncoding(value) != nil {
		return ErrEncoding
	}
	return nil
}

func encodedSize(flags policy.Disclosures) (int, error) {
	if err := checkDisclosures(flags); err != nil {
		return 0, err
	}
	count := 1
	if flags&policy.AuditorDisclosure != 0 {
		count++
	}
	if flags&policy.ControllerDisclosure != 0 {
		count++
	}
	return headerSize + count*envelope.EncodedSize, nil
}
