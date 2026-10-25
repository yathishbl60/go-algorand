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

package eval

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/notetree"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) openAssetPrivacyNoteTree(asset basics.AssetIndex, expected tree.Hash) (*notetree.Store, error) {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, policy.ErrMissing
	}
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return nil, err
	}
	if expected != identity {
		return nil, tree.ErrDomain
	}
	domain := tree.Domain{Genesis: cs.GenesisHash(), Asset: uint64(asset), Policy: identity}
	return notetree.Open(assetPrivacyTreeKV{cs}, domain)
}
