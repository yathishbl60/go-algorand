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

package shieldplan

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

func validate(domain tree.Domain, group [64]byte, debit uint64, outputs [2][64]byte) error {
	if domain.Genesis == [32]byte{} || domain.Asset == 0 || domain.Policy == [64]byte{} {
		return ErrConfig
	}
	if group == [64]byte{} || debit == 0 {
		return ErrStatement
	}
	if outputs[0] == [64]byte{} || outputs[1] == [64]byte{} || outputs[0] == outputs[1] {
		return ErrStatement
	}
	return nil
}
