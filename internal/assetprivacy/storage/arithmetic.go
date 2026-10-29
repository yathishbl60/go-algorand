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

package storage

func add(a, b uint64) (uint64, error) {
	if b > ^uint64(0)-a {
		return 0, ErrArithmetic
	}
	return a + b, nil
}

func subtract(a, b uint64) (uint64, error) {
	if a < b {
		return 0, ErrArithmetic
	}
	return a - b, nil
}

func multiply(a, b uint64) (uint64, error) {
	if a != 0 && b > ^uint64(0)/a {
		return 0, ErrArithmetic
	}
	return a * b, nil
}

func (o Obligation) Required(flat, perByte uint64) (uint64, error) {
	if err := o.Validate(); err != nil {
		return 0, err
	}
	if flat == 0 || perByte == 0 {
		return 0, ErrPrices
	}
	items, err := multiply(o.Items, flat)
	if err != nil {
		return 0, err
	}
	bytes, err := multiply(o.Bytes, perByte)
	if err != nil {
		return 0, err
	}
	return add(items, bytes)
}
