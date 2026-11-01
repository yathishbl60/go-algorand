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

package query

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/control"
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type AuthorizationSnapshot struct {
	Round, Asset              uint64
	PolicyID                  [64]byte
	Manager, Freeze, Clawback [32]byte
	DefaultFrozen             bool
	Keys                      keyauth.Root
	KeyCount                  uint64
	Eligibility               eligibility.Root
	EligibilityCount          uint64
	Targets                   *disclosure.Record
}

// KeySnapshot distinguishes registration membership from live role assignment.
type KeySnapshot struct {
	Round, Asset            uint64
	PolicyID                [64]byte
	Account                 [32]byte
	Role                    keys.Role
	PolicyAllowed, Assigned bool
	Root                    keyauth.Root
	Count                   uint64
	Record                  *keys.Record
	Witness                 *tree.Witness
}

// Missing Record/Witness never grants eligibility or proves nonmembership.
type EligibilitySnapshot struct {
	Round, Asset uint64
	PolicyID     [64]byte
	Account      [32]byte
	Frozen       bool
	Marker       *control.Record
	Root         eligibility.Root
	Count        uint64
	Record       *eligibility.Record
	Witness      *tree.Witness
}
