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

package libgoal

import (
	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
)

func (recovered *NativePrivacyRecoveredNote) PublicMetadata() NativePrivacyRecoveredOutput {
	if recovered == nil {
		return NativePrivacyRecoveredOutput{}
	}
	return NativePrivacyRecoveredOutput{Network: recovered.network,
		Asset: basics.AssetIndex(recovered.opening.Asset), Policy: recovered.opening.Policy,
		Index: recovered.index, Round: recovered.round, Transaction: recovered.transaction,
		Commitment: recovered.commitment}
}

// RecoveredWith exposes local key coverage only; never send this pair to RPC.
func (recovered *NativePrivacyRecoveredNote) RecoveredWith() (apiclient.NativePrivacyRole, uint64) {
	if recovered == nil {
		return 0, 0
	}
	return recovered.role, recovered.epoch
}

// Destroy drops this handle's fields. Go does not guarantee zeroization of heap,
// stack or earlier copies; callers must also dispose of explicit exported secrets.
func (recovered *NativePrivacyRecoveredNote) Destroy() {
	if recovered != nil {
		*recovered = NativePrivacyRecoveredNote{}
	}
}
