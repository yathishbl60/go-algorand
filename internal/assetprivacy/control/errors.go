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

package control

import "errors"

var (
	ErrDomain     = errors.New("invalid native privacy control domain")
	ErrRecord     = errors.New("invalid native privacy control record")
	ErrEpoch      = errors.New("invalid native privacy control epoch")
	ErrHistory    = errors.New("inconsistent native privacy control history")
	ErrStale      = errors.New("stale native privacy control state")
	ErrAuthority  = errors.New("native privacy control requires current freeze authority")
	ErrCapability = errors.New("native privacy freeze capability is not retained")
)
