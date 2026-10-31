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
	if err := b.validateOperation(); err != nil {
		return err
	}
	if (b.Asset == 0) != (b.Operation == Create) || b.Suite == [64]byte{} {
		return ErrHeader
	}
	if b.Operation != Register && (b.Role != 0 || b.Account != [32]byte{} || len(b.PublicKey) != 0) {
		return ErrFields
	}
	if b.Operation == Register {
		return b.validateRegistration()
	}
	if b.Operation == Adopt || b.Operation == Create {
		return b.validateDeclaration()
	}
	return b.validateExisting()
}

func (b Body) validateDeclaration() error {
	if b.Policy != [64]byte{} || b.ExpectedEpoch != 0 || b.Target != [32]byte{} || b.Frozen {
		return ErrFields
	}
	declaration := policy.Policy{Modes: b.Modes, Capabilities: b.Capabilities, Disclosures: b.Disclosures}
	if err := declaration.Validate(); err != nil {
		return err
	}
	if b.Operation == Adopt && !b.Modes.Allows(policy.Public) {
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
	return b.validateCleanup()
}

// CheckSender checks syntax, nonzero sender and registration account equality.
// It does not authenticate the account or establish current issuer authority.
func (b Body) CheckSender(sender [32]byte) error {
	if err := b.Validate(); err != nil {
		return err
	}
	if sender == [32]byte{} || (b.Operation == Register && b.Account != sender) {
		return ErrSender
	}
	return nil
}
