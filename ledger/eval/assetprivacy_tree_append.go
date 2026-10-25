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
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// AppendAssetPrivacyCommitment appends a verified output in the group-local COW.
// This helper performs no proof verification, policy authorization or fee charging.
func (cs *roundCowState) AppendAssetPrivacyCommitment(aidx basics.AssetIndex, policy, leaf tree.Hash) (uint32, tree.Hash, error) {
	state, err := cs.openAssetPrivacyNoteTree(aidx, policy)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	return state.AppendAtRound(leaf, uint64(cs.Round()), cs.proto.NativeAssetPrivacyRootHistory)
}

// AssetPrivacyMembership reads private path data for an already appended leaf.
func (cs *roundCowState) AssetPrivacyMembership(aidx basics.AssetIndex, policy tree.Hash, index uint32) (tree.Witness, error) {
	state, err := cs.OpenAssetPrivacyTree(aidx, policy)
	if err != nil {
		return tree.Witness{}, err
	}
	return state.Witness(index)
}
