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

package proofbridge

// VerificationObserver receives counters only, after all Go-side preflight.
// It cannot change validation results. Calls are synchronous; callers sharing
// an observer across concurrent Verify calls must synchronize that observer.
type VerificationObserver interface {
	BeforeNativePrivacyBundle()
	AfterNativePrivacyBundle(verified bool)
}

// WithObserver copies the verifier configuration, preserving its immutable pins.
func (v *Verifier) WithObserver(observer VerificationObserver) *Verifier {
	if v == nil {
		return nil
	}
	observed := *v
	observed.observer = observer
	return &observed
}

func (v *Verifier) verifyObservedBackend(request, bundle []byte) error {
	if v.observer != nil {
		v.observer.BeforeNativePrivacyBundle()
	}
	err := verifyBackend(request, bundle)
	if v.observer != nil {
		v.observer.AfterNativePrivacyBundle(err == nil)
	}
	return err
}
