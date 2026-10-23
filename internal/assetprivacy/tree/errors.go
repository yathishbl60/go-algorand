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

package tree

import "errors"

var (
	ErrCommitment = errors.New("reserved empty asset privacy commitment")
	ErrDomain     = errors.New("invalid asset privacy tree domain")
	ErrEncoding   = errors.New("invalid asset privacy tree record")
	ErrCapacity   = errors.New("asset privacy tree capacity exceeded")
	ErrIndex      = errors.New("asset privacy tree leaf index out of range")
	ErrMissing    = errors.New("asset privacy tree node missing")
	ErrStale      = errors.New("asset privacy tree snapshot is stale")
	ErrDisabled   = errors.New("native asset privacy is disabled")
)
