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
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationproof"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

// Work observations survive COW rollback and never merge during state commits.
func (cs *roundCowState) observeAssetPrivacyWork(work ledgercore.NativePrivacyWork) {
	if cs.nativePrivacyWorkObserver != nil {
		cs.nativePrivacyWorkObserver.AfterNativePrivacyWork(work)
	}
}

func (cs *roundCowState) BeforeNativePrivacyBundle() {
	cs.observeAssetPrivacyWork(ledgercore.NativePrivacyWork{BundleVerificationCalls: 1})
}

func (cs *roundCowState) AfterNativePrivacyBundle(verified bool) {
	if verified {
		cs.observeAssetPrivacyWork(ledgercore.NativePrivacyWork{BundleVerificationSuccesses: 1})
	}
}

// Counts the possession Verify API call, including any of its preflight failures.
func (cs *roundCowState) verifyObservedAssetPrivacyAdmin(key []byte, binding [64]byte, frame []byte) error {
	cs.observeAssetPrivacyWork(ledgercore.NativePrivacyWork{AdminPossessionCalls: 1})
	err := registrationproof.Verify(key, binding, frame)
	if err == nil {
		cs.observeAssetPrivacyWork(ledgercore.NativePrivacyWork{AdminPossessionSuccesses: 1})
	}
	return err
}
