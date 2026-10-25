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

package disclosure

func (f Flags) Validate() error {
	if f & ^(AuditorRequired|ControllerRequired) != 0 {
		return ErrFlags
	}
	return nil
}

func (r Record) Validate() error {
	if err := r.Domain.Validate(); err != nil {
		return err
	}
	if err := r.Flags.Validate(); err != nil {
		return err
	}
	if r.Epoch == 0 {
		return ErrEpoch
	}
	return checkTargets(r.Flags, r.Auditor, r.Controller)
}

func checkTargets(flags Flags, auditor, controller [32]byte) error {
	if (flags&AuditorRequired != 0) != (auditor != [32]byte{}) {
		return ErrTarget
	}
	if (flags&ControllerRequired != 0) != (controller != [32]byte{}) {
		return ErrTarget
	}
	return nil
}
