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

// Package storage meters native KV obligations; it grants no transaction authority.
package storage

import "errors"

const EncodedSize = 17
const keyPrefix = "aprv:storage:"
const minimumBytes = uint64(len(keyPrefix) + 32 + EncodedSize)

var ErrNamespace = errors.New("invalid native privacy storage namespace")
var ErrPayer = errors.New("native privacy storage requires a nonzero payer")
var ErrEncoding = errors.New("invalid native privacy storage obligation")
var ErrArithmetic = errors.New("native privacy storage obligation arithmetic failed")
var ErrPrices = errors.New("native privacy storage prices must be nonzero")

// Obligation includes its own live counter key/value in Items and Bytes.
// Byte counts include complete KV keys and values, not only user payloads.
type Obligation struct {
	Items uint64
	Bytes uint64
}

func (o Obligation) Validate() error {
	if o.Items == 0 || o.Bytes < minimumBytes || o.Bytes-minimumBytes < o.Items-1 {
		return ErrEncoding
	}
	return nil
}

func New(payer [32]byte) (Obligation, error) {
	if _, err := Key(payer); err != nil {
		return Obligation{}, err
	}
	return Obligation{Items: 1, Bytes: minimumBytes}, nil
}
