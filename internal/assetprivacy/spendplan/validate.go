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

package spendplan

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

func (c Config) validate() error {
	if c.Domain.Genesis == [32]byte{} || c.Domain.Asset == 0 || c.Domain.Policy == [64]byte{} {
		return ErrConfig
	}
	if c.EligibilityRoot.Digest == [64]byte{} {
		return ErrConfig
	}
	return nil
}

func (s Statement) validate() error {
	if !s.Kind.valid() || s.Group == [64]byte{} {
		return ErrStatement
	}
	if s.Inputs[0].Nullifier == s.Inputs[1].Nullifier {
		return ErrStatement
	}
	for _, input := range s.Inputs {
		if input.Root == [64]byte{} || input.Count == 0 || input.Count > tree.Capacity {
			return ErrStatement
		}
	}
	return s.checkFields()
}

func (s Statement) checkOutputs() error {
	if s.Kind.transfer() && len(s.Outputs) != 2 {
		return ErrStatement
	}
	if s.Kind.unshield() && (len(s.Outputs) != 0 || s.Debit != 0) {
		return ErrStatement
	}
	for _, output := range s.Outputs {
		if output == [64]byte{} {
			return ErrStatement
		}
	}
	if len(s.Outputs) == 2 && s.Outputs[0] == s.Outputs[1] {
		return ErrStatement
	}
	return nil
}
