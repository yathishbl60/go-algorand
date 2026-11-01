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
	"github.com/algorand/go-algorand/crypto"
	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/note"
)

// NativePrivacyRecoveredNote retains a local decrypted opening behind redacted
// formatting. Recovery alone establishes neither spending authority nor unspent
// status. Explicit secret accessors must never be copied into node RPC requests.
type NativePrivacyRecoveredNote struct {
	opening     note.Note
	network     crypto.Digest
	index       uint32
	round       basics.Round
	transaction transactions.Txid
	commitment  [64]byte
	role        apiclient.NativePrivacyRole
	epoch       uint64
}

// NativePrivacyRecoveredOutput contains public output identity only. It excludes
// value, account, owner, viewing role and viewing epoch from its serializable form.
type NativePrivacyRecoveredOutput struct {
	Network     crypto.Digest     `json:"network"`
	Asset       basics.AssetIndex `json:"asset-id"`
	Policy      [64]byte          `json:"policy-id"`
	Index       uint32            `json:"index"`
	Round       basics.Round      `json:"round"`
	Transaction transactions.Txid `json:"transaction-id"`
	Commitment  [64]byte          `json:"commitment"`
}
