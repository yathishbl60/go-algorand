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

package ledger

import (
	"path/filepath"

	"github.com/algorand/go-algorand/internal/assetprivacy/artifactstore"
	"github.com/algorand/go-algorand/ledger/eval"
)

// SetNativeAssetPrivacyArtifactProvider changes local data availability only.
// Current consensus pins and actual proof checking remain evaluator obligations.
func (l *Ledger) SetNativeAssetPrivacyArtifactProvider(provider eval.NativeAssetPrivacyArtifactProvider) {
	if l == nil {
		return
	}
	if provider == nil {
		l.nativePrivacyArtifacts.current.Store(nil)
		return
	}
	l.nativePrivacyArtifacts.current.Store(&nativePrivacyArtifactSnapshot{provider: provider})
}

// NativeAssetPrivacyArtifactProvider returns one immutable atomic configuration snapshot.
func (l *Ledger) NativeAssetPrivacyArtifactProvider() eval.NativeAssetPrivacyArtifactProvider {
	if l == nil {
		return nil
	}
	snapshot := l.nativePrivacyArtifacts.current.Load()
	if snapshot == nil {
		return nil
	}
	return snapshot.provider
}

// Configure before tracker replay, which may evaluate historical native transactions.
// Legacy string-only ledger paths have no root-genesis artifact directory.
func (l *Ledger) configureNativeAssetPrivacyArtifacts() {
	root := l.dirsAndPrefix.RootGenesisDir
	if root == "" {
		return
	}
	provider := artifactstore.NewDirectory(filepath.Join(root, "native-privacy", "registries"))
	l.SetNativeAssetPrivacyArtifactProvider(provider)
}
