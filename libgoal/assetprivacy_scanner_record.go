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
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (scanner *NativePrivacyScanner) scanRecord(value model.NativeAssetPrivacyOutput) (outputs.Record, error) {
	if value.Index > 1<<32-1 || value.Output > 1 || value.Disclosures != scanner.domain.Disclosures ||
		len(value.TransactionId) != 32 || len(value.Nonce) != 32 || len(value.Commitment) != 64 {
		return outputs.Record{}, ErrNativePrivacyScanPolicy
	}
	record := outputs.Record{Domain: tree.Domain{Genesis: scanner.domain.Network, Asset: uint64(scanner.domain.Asset), Policy: scanner.domain.Policy},
		Index: uint32(value.Index), Round: value.Round, Output: uint32(value.Output), Disclosures: policy.Disclosures(value.Disclosures), Recipient: value.Recipient}
	copy(record.Transaction[:], value.TransactionId)
	copy(record.Nonce[:], value.Nonce)
	copy(record.Commitment[:], value.Commitment)
	copyNativePrivacyScanDisclosures(&record, value)
	if err := record.Validate(); err != nil {
		return outputs.Record{}, err
	}
	return record, nil
}

func copyNativePrivacyScanDisclosures(record *outputs.Record, value model.NativeAssetPrivacyOutput) {
	if value.Auditor != nil {
		record.Auditor = *value.Auditor
	}
	if value.Controller != nil {
		record.Controller = *value.Controller
	}
}
