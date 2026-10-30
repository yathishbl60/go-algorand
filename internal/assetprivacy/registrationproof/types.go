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

// Package registrationproof frames and verifies candidate admin-key possession.
// A verified signature does not establish current account or issuer authority.
package registrationproof

import (
	"errors"

	"github.com/algorand/go-algorand/crypto"
)

const HeaderSize = 8
const MaxSize = HeaderSize + crypto.Falcon1024MaxSignatureSize
const messageDomain = "Algorand/AssetPrivacy/Candidate1/Registration/Falcon1024/"

var ErrFrame = errors.New("invalid native admin registration possession frame")
var ErrGroup = errors.New("native registration possession requires nonzero group context")
