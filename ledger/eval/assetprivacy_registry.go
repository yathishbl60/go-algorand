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
	"fmt"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

func (eval *BlockEvaluator) assetPrivacyRegistry(suite [64]byte) ([]byte, error) {
	selected, err := suitecatalog.Lookup(eval.proto, suite)
	if err != nil {
		return nil, err
	}
	ledger, ok := eval.l.(interface {
		NativeAssetPrivacyArtifactProvider() NativeAssetPrivacyArtifactProvider
	})
	if !ok {
		return nil, proofbridge.ErrUnavailable
	}
	provider := ledger.NativeAssetPrivacyArtifactProvider()
	if provider == nil {
		return nil, proofbridge.ErrUnavailable
	}
	return readAssetPrivacyRegistry(provider, selected.Pin().Digest)
}

func readAssetPrivacyRegistry(provider NativeAssetPrivacyArtifactProvider, digest [64]byte) ([]byte, error) {
	encoded, err := provider.Get(digest)
	if err != nil {
		return nil, fmt.Errorf("%w: local registry: %v", proofbridge.ErrUnavailable, err)
	}
	return encoded[:], nil
}
