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

package outputs

import "errors"

var (
	ErrDomain      = errors.New("invalid native privacy output domain")
	ErrEncoding    = errors.New("invalid native privacy output encoding")
	ErrDisclosures = errors.New("native privacy output disclosures do not match policy")
	ErrState       = errors.New("inconsistent native privacy output state")
	ErrStale       = errors.New("native privacy output snapshot is stale")
	ErrIndex       = errors.New("native privacy output index out of range")
	ErrPage        = errors.New("invalid native privacy output page")
)
