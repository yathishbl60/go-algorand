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
	"context"
	"errors"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
)

func (scanner *NativePrivacyScanner) recoverScanOutput(ctx context.Context, value model.NativeAssetPrivacyOutput) (*NativePrivacyRecoveredNote, error) {
	record, err := scanner.scanRecord(value)
	if err != nil {
		return nil, err
	}
	for _, key := range scanner.keys {
		if err = ctx.Err(); err != nil {
			return nil, err
		}
		if !nativePrivacyScanRolePresent(record, key.role) {
			continue
		}
		match, err := recoverNativePrivacyScanKey(record, key)
		if err != nil || match != nil {
			return match, err
		}
	}
	return nil, nil
}

func recoverNativePrivacyScanKey(record outputs.Record, key NativePrivacyViewingKey) (*NativePrivacyRecoveredNote, error) {
	opening, err := record.Recover(key.key, key.role, key.epoch)
	if errors.Is(err, envelope.ErrCiphertext) {
		return nil, nil
	}
	if err != nil {
		return nil, ErrNativePrivacyScanRecovery
	}
	return &NativePrivacyRecoveredNote{opening: opening, network: record.Domain.Genesis, index: record.Index,
		round: basics.Round(record.Round), transaction: transactions.Txid(record.Transaction), commitment: record.Commitment,
		role: nativePrivacyScanPublicRole(key.role), epoch: key.epoch}, nil
}

func nativePrivacyScanRolePresent(record outputs.Record, role envelope.Role) bool {
	return role == envelope.Recipient || role == envelope.Auditor && len(record.Auditor) != 0 || role == envelope.Controller && len(record.Controller) != 0
}

func nativePrivacyScanPublicRole(role envelope.Role) apiclient.NativePrivacyRole {
	switch role {
	case envelope.Recipient:
		return apiclient.NativePrivacyRecipientView
	case envelope.Auditor:
		return apiclient.NativePrivacyAuditorView
	default:
		return apiclient.NativePrivacyControllerView
	}
}
