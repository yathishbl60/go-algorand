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

func (kind Kind) arity() (int, int, error) {
	switch kind {
	case Transfer:
		return 2, 2, nil
	case Unshield:
		return 2, 0, nil
	case PartialPrivate:
		return 1, 2, nil
	case FullPrivate, PartialPublic:
		return 1, 1, nil
	case FullPublic:
		return 1, 0, nil
	default:
		return 0, 0, ErrOperation
	}
}

func (b Body) checkConversion() error {
	if (b.Debit > 0) != (b.DebitSource != [32]byte{}) || (b.Credit > 0) != (b.CreditReceiver != [32]byte{}) {
		return ErrConversion
	}
	switch b.Operation {
	case Transfer:
		return nil
	case Unshield, PartialPublic, FullPublic:
		if b.Debit != 0 || b.Credit == 0 {
			return ErrConversion
		}
	case PartialPrivate, FullPrivate:
		if b.Debit != 0 || b.Credit != 0 {
			return ErrConversion
		}
	}
	return nil
}
