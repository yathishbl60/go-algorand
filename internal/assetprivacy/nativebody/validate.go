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

func (b Body) validateHeader() error {
	if _, _, err := b.Operation.arity(); err != nil {
		return err
	}
	if !b.validMode() {
		return ErrMode
	}
	if b.Asset == 0 || b.Policy == [64]byte{} || b.Suite == [64]byte{} {
		return ErrHeader
	}
	return b.checkConversion()
}

// Validate checks canonical public body semantics only, not native authorization.
func (b Body) Validate() error {
	if err := b.validateHeader(); err != nil {
		return err
	}
	inputs, outputs, _ := b.Operation.arity()
	if len(b.Inputs) != inputs || len(b.Outputs) != outputs {
		return ErrArity
	}
	if err := validateInputs(b.Inputs); err != nil {
		return err
	}
	if err := validateOutputs(b.Outputs); err != nil {
		return err
	}
	if encodedSize(b) > MaxSize {
		return ErrEncoding
	}
	return b.validateParticipants()
}

// CheckSender binds a positive public debit to the nonzero outer transaction sender.
// Equality is an identity comparison and never authenticates signatures or consent.
func (b Body) CheckSender(sender [32]byte) error {
	if err := b.Validate(); err != nil {
		return err
	}
	if sender == [32]byte{} || (b.Debit > 0 && b.DebitSource != sender) {
		return ErrSender
	}
	return nil
}
