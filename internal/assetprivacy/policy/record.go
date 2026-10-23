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

package policy

import "errors"

var ErrRecord = errors.New("invalid native privacy policy record")
var ErrMissing = errors.New("native privacy policy does not exist")
var ErrManager = errors.New("privacy policy requires the current asset manager")

// Record preserves the immutable initial authority declaration and suite identity.
type Record struct {
	Policy              Policy
	InitialCapabilities Capabilities
	Suite               [64]byte
	Created             uint64
}

func (r Record) Validate() error {
	if err := r.Policy.Validate(); err != nil {
		return err
	}
	initial := r.Policy
	initial.Capabilities = r.InitialCapabilities
	if err := initial.Validate(); err != nil {
		return err
	}
	if r.Suite == [64]byte{} || r.Policy.Capabilities & ^r.InitialCapabilities != 0 {
		return ErrRecord
	}
	return nil
}

// CheckRecordUpdate preserves the policy identity while allowing relinquishment.
func CheckRecordUpdate(enabled bool, before, after Record) error {
	if !enabled {
		return ErrDisabled
	}
	if err := before.Validate(); err != nil {
		return err
	}
	if err := after.Validate(); err != nil {
		return err
	}
	if before.InitialCapabilities != after.InitialCapabilities || before.Suite != after.Suite || before.Created != after.Created {
		return ErrImmutable
	}
	return CheckUpdate(enabled, before.Policy, after.Policy)
}
