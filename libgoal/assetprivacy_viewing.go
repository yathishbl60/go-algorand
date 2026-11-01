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
	"crypto/mlkem"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
)

// NativePrivacyViewingKey is a local historical role/epoch key, never an RPC input.
// Expanded keys remain in Go-managed memory; this handle makes no zeroization
// guarantee. Copies share the immutable standard-library decapsulation key.
type NativePrivacyViewingKey struct {
	key      *mlkem.DecapsulationKey768
	role     envelope.Role
	epoch    uint64
	identity [64]byte
}

// RestoreNativePrivacyViewingKey restores a uniformly random 64-byte d||z seed.
// Keep seed stable until return; the caller owns and clears its input buffer.
// Restoring a viewing key proves neither registration nor current assignment.
func RestoreNativePrivacyViewingKey(seed []byte, role apiclient.NativePrivacyRole, epoch uint64) (NativePrivacyViewingKey, error) {
	viewingRole := nativePrivacyViewingRole(role)
	if viewingRole == 0 || epoch == 0 || len(seed) != mlkem.SeedSize {
		return NativePrivacyViewingKey{}, ErrNativePrivacyScanConfig
	}
	key, err := mlkem.NewDecapsulationKey768(seed)
	if err != nil {
		return NativePrivacyViewingKey{}, ErrNativePrivacyScanConfig
	}
	public := key.EncapsulationKey().Bytes()
	if envelope.CheckPublicKey(public) != nil {
		return NativePrivacyViewingKey{}, ErrNativePrivacyScanConfig
	}
	return NativePrivacyViewingKey{key: key, role: viewingRole, epoch: epoch, identity: nativePrivacyViewingIdentity(role, epoch, public)}, nil
}

func nativePrivacyViewingRole(role apiclient.NativePrivacyRole) envelope.Role {
	switch role {
	case apiclient.NativePrivacyRecipientView:
		return envelope.Recipient
	case apiclient.NativePrivacyAuditorView:
		return envelope.Auditor
	case apiclient.NativePrivacyControllerView:
		return envelope.Controller
	}
	return 0
}
