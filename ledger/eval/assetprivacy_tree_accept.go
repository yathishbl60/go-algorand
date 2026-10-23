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

// AcceptAssetPrivacyRoot applies the consensus round-age window to root and count.
// It performs neither proof verification nor policy/transaction authorization.
func (cs *roundCowState) AcceptAssetPrivacyRoot(aidx basics.AssetIndex, policy, root tree.Hash, count uint64) (bool, error) {
	state, err := cs.OpenAssetPrivacyTree(aidx, policy)
	if err != nil {
		return false, err
	}
	return state.AcceptRoot(root, count, uint64(cs.Round()), cs.proto.NativeAssetPrivacyRootHistory)
}
