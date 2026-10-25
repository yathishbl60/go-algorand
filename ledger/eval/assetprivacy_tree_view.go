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
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// AssetPrivacyNoteTreeView exposes no append, replacement or deletion capability.
type AssetPrivacyNoteTreeView struct{ store *notetree.Store }

func (cs *roundCowState) OpenAssetPrivacyTree(asset basics.AssetIndex, policy tree.Hash) (*AssetPrivacyNoteTreeView, error) {
	store, err := cs.openAssetPrivacyNoteTree(asset, policy)
	if err != nil {
		return nil, err
	}
	return &AssetPrivacyNoteTreeView{store}, nil
}

func (v *AssetPrivacyNoteTreeView) Root() tree.Hash { return v.store.Root() }
func (v *AssetPrivacyNoteTreeView) Count() uint64   { return v.store.Count() }
func (v *AssetPrivacyNoteTreeView) Witness(index uint32) (tree.Witness, error) {
	return v.store.Witness(index)
}
func (v *AssetPrivacyNoteTreeView) AcceptRoot(root tree.Hash, count, round, slots uint64) (bool, error) {
	return v.store.AcceptRoot(root, count, round, slots)
}
