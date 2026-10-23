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

package accounting

import "errors"

// Errors classify local accounting failures; they are not protocol wire codes.
var (
	ErrDisabled       = errors.New("native asset privacy is disabled")
	ErrSupply         = errors.New("private supply exceeds the asset total")
	ErrPublicSupply   = errors.New("shield amount exceeds aggregate public supply")
	ErrPrivateSupply  = errors.New("unshield amount exceeds aggregate private supply")
	ErrOutstanding    = errors.New("private supply remains outstanding")
	ErrCreatorHolding = errors.New("creator does not hold the entire asset supply")
	ErrAssetID        = errors.New("privacy supply requires a nonzero asset ID")
	ErrEncoding       = errors.New("invalid native private-supply record")
)
