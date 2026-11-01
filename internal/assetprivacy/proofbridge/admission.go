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

import "fmt"

// ErrBusy is local contention, never invalid-proof or invalid-peer evidence.
var ErrBusy = fmt.Errorf("%w: another native bundle verification is active", ErrResource)

// One process-wide lease covers payload copying, request assembly and backend
// verification. It is not a memory quota or a per-peer fairness mechanism.
var nativeVerificationLease = make(chan struct{}, 1)

func acquireNativeVerification() error {
	select {
	case nativeVerificationLease <- struct{}{}:
		return nil
	default:
		return ErrBusy
	}
}

func releaseNativeVerification() { <-nativeVerificationLease }
