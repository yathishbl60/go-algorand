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
	"bytes"
	"context"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
)

func (scanner *NativePrivacyScanner) publicScanPage(ctx context.Context, options apiclient.NativePrivacyOutputOptions) (model.NativeAssetPrivacyOutputPage, error) {
	if err := scanner.scanNetwork(ctx); err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	page, err := scanner.client.Outputs(ctx, scanner.domain.Asset, options)
	if err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	if !bytes.Equal(page.PolicyId, scanner.domain.Policy[:]) {
		return model.NativeAssetPrivacyOutputPage{}, ErrNativePrivacyScanPolicy
	}
	if err = scanner.scanPolicy(ctx, basics.Round(page.Round)); err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	return page, nil
}

func (scanner *NativePrivacyScanner) scanNetwork(ctx context.Context) error {
	network, err := scanner.client.GenesisHash(ctx)
	if err != nil {
		return err
	}
	if network != scanner.domain.Network {
		return ErrNativePrivacyScanNetwork
	}
	return nil
}

func (scanner *NativePrivacyScanner) scanPolicy(ctx context.Context, round basics.Round) error {
	snapshot, err := scanner.client.Snapshot(ctx, scanner.domain.Asset, &round)
	if err != nil {
		return err
	}
	if !snapshot.Exists || snapshot.Policy == nil || snapshot.Retirement != nil || snapshot.Closure != nil ||
		!bytes.Equal(snapshot.Policy.PolicyId, scanner.domain.Policy[:]) || snapshot.Policy.Disclosures != scanner.domain.Disclosures {
		return ErrNativePrivacyScanPolicy
	}
	return nil
}
