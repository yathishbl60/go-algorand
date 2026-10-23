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
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type assetPrivacyOutputState struct {
	stream      *outputs.Store
	commitments *tree.Tree
}

// openAssetPrivacyOutputs derives the domain from authoritative current policy.
func (cs *roundCowState) openAssetPrivacyOutputs(asset basics.AssetIndex) (*assetPrivacyOutputState, error) {
	record, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, policy.ErrMissing
	}
	identity, err := record.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return nil, err
	}
	domain := tree.Domain{Genesis: cs.GenesisHash(), Asset: uint64(asset), Policy: identity}
	return cs.loadAssetPrivacyOutputs(domain, record.Policy.Disclosures)
}

func (cs *roundCowState) loadAssetPrivacyOutputs(domain tree.Domain, flags policy.Disclosures) (*assetPrivacyOutputState, error) {
	stream, err := outputs.Open(assetPrivacyTreeKV{cs}, domain, flags)
	if err != nil {
		return nil, err
	}
	commitments, err := tree.Open(assetPrivacyTreeKV{cs}, domain)
	if err != nil {
		return nil, err
	}
	if stream.Count() != commitments.Count() || stream.LastRound() > uint64(cs.Round()) {
		return nil, outputs.ErrState
	}
	return &assetPrivacyOutputState{stream, commitments}, nil
}
