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
	"github.com/algorand/go-algorand/internal/assetprivacy/nullifier"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) checkAssetPrivacySpendInputs(asset basics.AssetIndex, policy tree.Hash, inputs [2]spendplan.Input) error {
	for _, input := range inputs {
		if err := cs.checkAssetPrivacySpendInput(asset, policy, input); err != nil {
			return err
		}
	}
	return nil
}

func (cs *roundCowState) checkAssetPrivacySpendInput(asset basics.AssetIndex, policy tree.Hash, input spendplan.Input) error {
	accepted, err := cs.AcceptAssetPrivacyRoot(asset, policy, input.Root, input.Count)
	if err != nil {
		return err
	}
	if !accepted {
		return tree.ErrHistory
	}
	spent, err := cs.HasAssetPrivacyNullifier(asset, input.Nullifier)
	if err != nil {
		return err
	}
	if spent {
		return nullifier.ErrSpent
	}
	return nil
}
