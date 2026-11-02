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
	algodclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/stretchr/testify/require"
	"net/http"
	"time"
)

// This checks one owner registry across a peer restart, not full wallet restoration.
func (f *ownerFixture) checkPeerAndRestart(round basics.Round, expected model.NativeAssetPrivacyKeySnapshot) {
	f.checkPeerState(f.peerQueries(round), expected)
	controller, err := f.fixture.GetNodeController("Node")
	require.NoError(f.t, err)
	require.NoError(f.t, controller.StopAlgod())
	_, err = f.fixture.StartNode(controller.GetDataDir())
	require.NoError(f.t, err)
	f.checkPeerState(f.peerQueries(round), expected)
	f.t.Log("native owner key/root/count and public supply agree on peer before and after restart")
}

func (f *ownerFixture) peerQueries(round basics.Round) *algodclient.NativePrivacyClient {
	controller, err := f.fixture.GetNodeController("Node")
	require.NoError(f.t, err)
	client := f.fixture.GetAlgodClientForController(controller)
	_, err = client.WaitForRound(round, 30*time.Second)
	require.NoError(f.t, err)
	queries, err := client.NativePrivacyQueries(&http.Client{Timeout: 30 * time.Second})
	require.NoError(f.t, err)
	return queries
}

func (f *ownerFixture) checkPeerState(queries *algodclient.NativePrivacyClient, expected model.NativeAssetPrivacyKeySnapshot) {
	key, err := queries.Key(context.Background(), f.asset, f.account.Address,
		algodclient.NativePrivacyRecipientSpend, nil)
	require.NoError(f.t, err)
	require.True(f.t, key.Registered)
	require.Equal(f.t, expected.Record, key.Record)
	require.Equal(f.t, expected.KeyRoot, key.KeyRoot)
	require.Equal(f.t, expected.KeyCount, key.KeyCount)
	snapshot, err := queries.Snapshot(context.Background(), f.asset, nil)
	require.NoError(f.t, err)
	require.Zero(f.t, snapshot.PrivateSupply)
	require.EqualValues(f.t, 100, snapshot.Total)
	require.NotNil(f.t, snapshot.Policy)
	require.Equal(f.t, f.policy[:], snapshot.Policy.PolicyId)
}
