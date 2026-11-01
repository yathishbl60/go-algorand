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

package ledgercore

import "github.com/algorand/go-algorand/data/basics"

// NativePrivacyWork is non-consensus diagnostic metadata retained on rollback.
// ReservedUnits are admitted work reservations, including cleanup units, not
// actual proof checks. Bundle calls reach the backend after Go preflight. Admin
// calls enter registrationproof.Verify and can fail its preflight. Successes
// establish only the corresponding verification result, not transaction success.
type NativePrivacyWork struct {
	ReservedUnits               uint64
	BundleVerificationCalls     uint64
	BundleVerificationSuccesses uint64
	AdminPossessionCalls        uint64
	AdminPossessionSuccesses    uint64
}

// NativePrivacyWorkObserver receives numeric diagnostic deltas only.
// Observations cannot authorize, approve, or change a validation result.
type NativePrivacyWorkObserver interface {
	AfterNativePrivacyWork(NativePrivacyWork)
}

// Add accumulates diagnostics without wraparound; it does not change work limits.
func (work *NativePrivacyWork) Add(delta NativePrivacyWork) {
	work.ReservedUnits = basics.AddSaturate(work.ReservedUnits, delta.ReservedUnits)
	work.BundleVerificationCalls = basics.AddSaturate(work.BundleVerificationCalls, delta.BundleVerificationCalls)
	work.BundleVerificationSuccesses = basics.AddSaturate(work.BundleVerificationSuccesses, delta.BundleVerificationSuccesses)
	work.AdminPossessionCalls = basics.AddSaturate(work.AdminPossessionCalls, delta.AdminPossessionCalls)
	work.AdminPossessionSuccesses = basics.AddSaturate(work.AdminPossessionSuccesses, delta.AdminPossessionSuccesses)
}

func (work NativePrivacyWork) IsZero() bool { return work == (NativePrivacyWork{}) }
