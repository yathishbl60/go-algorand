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
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) assetPrivacyTransactionOutputs(body nativebody.Body, bodies AssetPrivacySpendBodies) ([]outputs.Record, error) {
	context, err := cs.assetPrivacyBindBodies(body.Suite, bodies)
	if err != nil {
		return nil, err
	}
	state, err := cs.openAssetPrivacyOutputs(basics.AssetIndex(body.Asset))
	if err != nil {
		return nil, err
	}
	return assetPrivacyBodyOutputRecords(body, state.stream.Count(), context, cs.GenesisHash())
}

func assetPrivacyBodyOutputRecords(body nativebody.Body, count uint64, context assetPrivacyBodyContext, genesis [32]byte) ([]outputs.Record, error) {
	if count > tree.Capacity || uint64(len(body.Outputs)) > tree.Capacity-count {
		return nil, tree.ErrCapacity
	}
	domain := tree.Domain{Genesis: genesis, Asset: body.Asset, Policy: body.Policy}
	records := make([]outputs.Record, len(body.Outputs))
	for i, output := range body.Outputs {
		records[i] = outputs.Record{Domain: domain, Index: uint32(count + uint64(i)),
			Round: context.round, Transaction: context.transaction, Output: uint32(i),
			Nonce: output.Nonce, Commitment: output.Commitment, Disclosures: output.Disclosures,
			Recipient: output.Recipient, Auditor: output.Auditor, Controller: output.Controller}
	}
	return records, nil
}
