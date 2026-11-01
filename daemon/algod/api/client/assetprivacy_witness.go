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

package client

import (
	"context"
	"fmt"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
)

func (client *NativePrivacyClient) NoteWitness(ctx context.Context, asset basics.AssetIndex, index uint32, round *basics.Round) (model.NativeAssetPrivacyNoteWitness, error) {
	if asset == 0 {
		return model.NativeAssetPrivacyNoteWitness{}, ErrNativePrivacyRequest
	}
	var result model.NativeAssetPrivacyNoteWitness
	path := nativePrivacyPath(asset, fmt.Sprintf("/notes/%d/witness", index))
	if err := client.get(ctx, path, nativePrivacyRoundQuery(round), &result); err != nil {
		return model.NativeAssetPrivacyNoteWitness{}, err
	}
	if err := nativePrivacyScope(asset, round, result.Round, result.AssetId); err != nil {
		return model.NativeAssetPrivacyNoteWitness{}, err
	}
	if result.Index != uint64(index) || len(result.PolicyId) != 64 || !nativePrivacyPathShape(result.Root, result.Leaf, result.Siblings, result.Count, result.Index) {
		return model.NativeAssetPrivacyNoteWitness{}, ErrNativePrivacyResponse
	}
	return result, nil
}

func nativePrivacyPathShape(root, leaf []byte, siblings [][]byte, count, index uint64) bool {
	if len(root) != 64 || len(leaf) != 64 || count == 0 || count > 1<<32 || index >= count || len(siblings) != 32 {
		return false
	}
	for _, sibling := range siblings {
		if len(sibling) != 64 {
			return false
		}
	}
	return true
}
