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

// Change replaces the complete old footprint with the complete new footprint.
// An absent leg ignores its byte length; present entries require a nonempty key.
// The own counter is not passed here. Only authenticated final cleanup can remove
// a counter proven to contain precisely its own footprint and no other entries.
func (o Obligation) Change(keyBytes, beforeBytes uint64, existed bool, afterBytes uint64, present bool) (Obligation, error) {
	if err := o.checkChange(keyBytes, existed || present); err != nil {
		return Obligation{}, err
	}
	var err error
	if existed {
		o, err = o.remove(keyBytes, beforeBytes)
		if err != nil {
			return Obligation{}, err
		}
	}
	if present {
		o, err = o.insert(keyBytes, afterBytes)
		if err != nil {
			return Obligation{}, err
		}
	}
	return o, o.Validate()
}

func (o Obligation) remove(keyBytes, valueBytes uint64) (Obligation, error) {
	bytes, err := add(keyBytes, valueBytes)
	if err != nil {
		return Obligation{}, err
	}
	o.Items, err = subtract(o.Items, 1)
	if err != nil {
		return Obligation{}, err
	}
	o.Bytes, err = subtract(o.Bytes, bytes)
	if err != nil {
		return Obligation{}, err
	}
	return o, o.Validate()
}

func (o Obligation) checkChange(keyBytes uint64, active bool) error {
	if err := o.Validate(); err != nil {
		return err
	}
	if active && keyBytes == 0 {
		return ErrArithmetic
	}
	return nil
}

func (o Obligation) insert(keyBytes, valueBytes uint64) (Obligation, error) {
	bytes, err := add(keyBytes, valueBytes)
	if err != nil {
		return Obligation{}, err
	}
	o.Items, err = add(o.Items, 1)
	if err != nil {
		return Obligation{}, err
	}
	o.Bytes, err = add(o.Bytes, bytes)
	return o, err
}
