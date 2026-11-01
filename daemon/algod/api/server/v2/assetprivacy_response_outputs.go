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

package v2

import (
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func nativePrivacyPageResponse(value query.OutputPage) (model.NativeAssetPrivacyOutputPage, error) {
	token, err := value.NextCursor()
	if err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	response := model.NativeAssetPrivacyOutputPage{Round: value.Round, AssetId: value.Asset, PolicyId: value.PolicyID[:],
		Root: value.Root[:], Count: value.Count, Next: value.Next, Outputs: make([]model.NativeAssetPrivacyOutput, len(value.Records))}
	for i, output := range value.Records {
		response.Outputs[i] = nativePrivacyOutputResponse(output)
	}
	if token != "" {
		response.NextToken = &token
	}
	return response, nil
}

func nativePrivacyOutputResponse(value outputs.Record) model.NativeAssetPrivacyOutput {
	response := model.NativeAssetPrivacyOutput{Index: uint64(value.Index), Round: value.Round, TransactionId: value.Transaction[:],
		Nonce: value.Nonce[:], Output: uint64(value.Output), Commitment: value.Commitment[:],
		Disclosures: uint64(value.Disclosures), Recipient: value.Recipient}
	if len(value.Auditor) != 0 {
		response.Auditor = &value.Auditor
	}
	if len(value.Controller) != 0 {
		response.Controller = &value.Controller
	}
	return response
}

func nativePrivacyWitnessResponse(value query.NoteWitness) model.NativeAssetPrivacyNoteWitness {
	response := model.NativeAssetPrivacyNoteWitness{Round: value.Round, AssetId: value.Asset, PolicyId: value.PolicyID[:],
		Root: value.Witness.Root[:], Count: value.Witness.Count, Index: uint64(value.Witness.Index), Leaf: value.Witness.Leaf[:],
		Siblings: make([][]byte, len(value.Witness.Siblings))}
	for i := range response.Siblings {
		response.Siblings[i] = value.Witness.Siblings[i][:]
	}
	return response
}
