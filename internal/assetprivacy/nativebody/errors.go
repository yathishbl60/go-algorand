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

import "errors"

var (
	ErrEncoding    = errors.New("invalid native privacy body encoding or length")
	ErrHeader      = errors.New("invalid native privacy body domain")
	ErrOperation   = errors.New("unsupported native privacy body operation")
	ErrMode        = errors.New("unsupported native privacy body mode")
	ErrArity       = errors.New("native privacy body input/output counts mismatch")
	ErrConversion  = errors.New("invalid native privacy public conversion")
	ErrInput       = errors.New("invalid native privacy body input or duplicate nullifier")
	ErrOutput      = errors.New("invalid native privacy body output or duplicate commitment")
	ErrDisclosures = errors.New("invalid native privacy body disclosure roles")
	ErrSender      = errors.New("native privacy debit source does not match sender")
)
