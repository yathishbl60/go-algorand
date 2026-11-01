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
	"errors"

	"github.com/algorand/go-algorand/crypto"
	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
)

var ErrNativePrivacyScanConfig = errors.New("invalid native privacy scanner configuration")
var ErrNativePrivacyScanNetwork = errors.New("native privacy scanner network mismatch")
var ErrNativePrivacyScanPolicy = errors.New("native privacy scanner policy mismatch")
var ErrNativePrivacyScanCheckpoint = errors.New("native privacy scanner checkpoint mismatch")
var ErrNativePrivacyScanRecovery = errors.New("native privacy scanner recovered an invalid opening")
var ErrNativePrivacyScanStorage = errors.New("native privacy scan pages require explicit secure opening storage")
var ErrNativePrivacyScanComplete = errors.New("native privacy snapshot scan is complete")

const NativePrivacyScanMaxKeys = 64

type NativePrivacyScanDomain struct {
	Network     crypto.Digest     `json:"network"`
	Asset       basics.AssetIndex `json:"asset"`
	Policy      [64]byte          `json:"policy"`
	Disclosures uint64            `json:"disclosures"`
}

// NativePrivacyScanCheckpoint is local, wallet-correlating progress metadata.
// It requires authenticated local storage; its key-set digest is never sent to RPC.
type NativePrivacyScanCheckpoint struct {
	NativePrivacyScanDomain
	Version   uint8        `json:"version"`
	KeySet    [64]byte     `json:"key-set"`
	Round     basics.Round `json:"round"`
	Root      [64]byte     `json:"root"`
	Count     uint64       `json:"count"`
	Next      uint64       `json:"next"`
	NextToken *string      `json:"next-token,omitempty"`
}

// NativePrivacyScanPage must be persisted atomically: openings before checkpoint.
// Recovery alone establishes no finality, unspent state, or spending authority.
type NativePrivacyScanPage struct {
	Notes      []*NativePrivacyRecoveredNote
	Checkpoint NativePrivacyScanCheckpoint
}

type NativePrivacyScanner struct {
	client   *apiclient.NativePrivacyClient
	domain   NativePrivacyScanDomain
	keys     []NativePrivacyViewingKey
	coverage [64]byte
}
