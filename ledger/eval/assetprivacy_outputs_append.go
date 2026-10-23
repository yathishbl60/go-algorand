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
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// AppendAssetPrivacyOutput atomically couples an encrypted record and its leaf.
// The record must match the derived domain, next index, current round and policy.
// Callers must verify proofs, disclosures, authority, fees and storage funding.
func (cs *roundCowState) AppendAssetPrivacyOutput(asset basics.AssetIndex, record outputs.Record) (uint32, tree.Hash, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return 0, tree.Hash{}, tree.ErrDisabled
	}
	child := cs.child(0)
	defer child.recycle()
	index, root, err := child.appendAssetPrivacyOutput(asset, record)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	child.commitToParent()
	return index, root, nil
}

func (cs *roundCowState) appendAssetPrivacyOutput(asset basics.AssetIndex, record outputs.Record) (uint32, tree.Hash, error) {
	if record.Round != uint64(cs.Round()) {
		return 0, tree.Hash{}, outputs.ErrState
	}
	state, err := cs.openAssetPrivacyOutputs(asset)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	if err = state.stream.CheckAppend(record); err != nil {
		return 0, tree.Hash{}, err
	}
	index, root, err := state.commitments.AppendAtRound(record.Commitment, record.Round, cs.proto.NativeAssetPrivacyRootHistory)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	if err = state.stream.Append(record); err != nil {
		return 0, tree.Hash{}, err
	}
	return index, root, nil
}
