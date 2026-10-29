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

package lifecyclebody

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

func (b Body) Validate() error {
	if b.Operation.encodedSize() == 0 {
		return ErrOperation
	}
	if b.Asset == 0 || b.Suite == [64]byte{} {
		return ErrHeader
	}
	if b.Operation == Adopt {
		return b.validateAdoption()
	}
	return b.validateExisting()
}

func (b Body) validateAdoption() error {
	if b.Policy != [64]byte{} || b.ExpectedEpoch != 0 || b.Target != [32]byte{} || b.Frozen {
		return ErrFields
	}
	declaration := policy.Policy{Modes: b.Modes, Capabilities: b.Capabilities, Disclosures: b.Disclosures}
	if err := declaration.Validate(); err != nil {
		return err
	}
	if !b.Modes.Allows(policy.Public) {
		return policy.ErrPublicRequired
	}
	required := b.Disclosures&policy.AuditorDisclosure != 0
	if required != (b.Auditor != [32]byte{}) {
		return ErrFields
	}
	return nil
}

func (b Body) validateExisting() error {
	if b.Policy == [64]byte{} {
		return ErrHeader
	}
	if b.Modes != 0 || b.Capabilities != 0 || b.Disclosures != 0 {
		return ErrFields
	}
	if b.Operation == Assign && (b.Target != [32]byte{} || b.Frozen) {
		return ErrFields
	}
	if b.Operation == Freeze && (b.Target == [32]byte{} || b.Auditor != [32]byte{}) {
		return ErrFields
	}
	return nil
}

// CheckSender checks syntax and nonzero identity, not account/PQ authorization.
func (b Body) CheckSender(sender [32]byte) error {
	if err := b.Validate(); err != nil {
		return err
	}
	if sender == [32]byte{} {
		return ErrSender
	}
	return nil
}
