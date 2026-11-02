//go:build algorand_native_privacy && algorand_native_privacy_stark

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

package assetprivacy

import (
	"context"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/note"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/protocol"
	"github.com/stretchr/testify/require"
	"net/http"
	"time"
)

func (f *ownerFixture) createAsset() {
	account := f.account.Address.String()
	tx, err := f.fixture.LibGoalClient.MakeUnsignedAssetCreateTx(100, false, account, "", "", "", "NAP", "OwnerE2E", "", nil, 0)
	require.NoError(f.t, err)
	pending := f.submit(f.fill(tx, 100_000))
	require.NotNil(f.t, pending.AssetIndex)
	f.asset = *pending.AssetIndex
}

func (f *ownerFixture) adoptPolicy() {
	body := lifecyclebody.Body{Operation: lifecyclebody.Adopt, Asset: uint64(f.asset),
		Suite: f.suite.ID, Modes: policy.Modes(policy.Public)}
	f.submit(f.lifecycleTransaction(body))
	var err error
	f.queries, err = f.fixture.LibGoalClient.NativePrivacyQueries(&http.Client{Timeout: 30 * time.Second})
	require.NoError(f.t, err)
	genesis, err := f.queries.GenesisHash(context.Background())
	require.NoError(f.t, err)
	f.network = [32]byte(genesis)
	f.loadPolicyIdentity()
	var secret [32]byte
	for i := range secret {
		secret[i] = 51
	}
	f.owner, err = note.Owner(f.network, secret)
	require.NoError(f.t, err)
}

func (f *ownerFixture) loadPolicyIdentity() {
	snapshot, err := f.queries.Snapshot(context.Background(), f.asset, nil)
	require.NoError(f.t, err)
	require.NotNil(f.t, snapshot.Policy)
	require.EqualValues(f.t, policy.Public, snapshot.Policy.Modes)
	require.Zero(f.t, snapshot.PrivateSupply)
	require.Len(f.t, snapshot.Policy.PolicyId, 64)
	f.policy = [64]byte(snapshot.Policy.PolicyId)
}

func (f *ownerFixture) lifecycleTransaction(body lifecyclebody.Body) transactions.Transaction {
	encoded, err := lifecyclebody.Encode(body)
	require.NoError(f.t, err)
	tx := transactions.Transaction{Type: protocol.NativeAssetPrivacyTx}
	tx.NativeAssetPrivacy = encoded
	return f.fill(tx, 100_000_000)
}
