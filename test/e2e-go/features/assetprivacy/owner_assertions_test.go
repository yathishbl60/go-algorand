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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/stretchr/testify/require"
	"slices"
)

func (f *ownerFixture) keySnapshot() model.NativeAssetPrivacyKeySnapshot {
	key, err := f.queries.Key(context.Background(), f.asset, f.account.Address,
		algodclient.NativePrivacyRecipientSpend, nil)
	require.NoError(f.t, err)
	return key
}

func (f *ownerFixture) receiverBalance() uint64 {
	balance, err := f.fixture.LibGoalClient.GetBalance(f.receiver.String())
	require.NoError(f.t, err)
	return balance
}

func (f *ownerFixture) rejectInvalidGroup(group []transactions.SignedTxn, invalid []byte) {
	before, balance := f.keySnapshot(), f.receiverBalance()
	require.False(f.t, before.Registered)
	bad := slices.Clone(group)
	bad[1].NativeAssetPrivacyProof = invalid
	err := f.fixture.LibGoalClient.BroadcastTransactionGroup(bad)
	require.ErrorContains(f.t, err, proofbridge.ErrRejected.Error())
	after := f.keySnapshot()
	require.False(f.t, after.Registered)
	require.Equal(f.t, before.Record, after.Record)
	require.Equal(f.t, before.KeyRoot, after.KeyRoot)
	require.Equal(f.t, before.KeyCount, after.KeyCount)
	require.Equal(f.t, balance, f.receiverBalance(), "preceding payment must roll back")
}

func (f *ownerFixture) acceptRegistration(group []transactions.SignedTxn) {
	balance := f.receiverBalance()
	require.NoError(f.t, f.fixture.LibGoalClient.BroadcastTransactionGroup(group))
	pending := f.waitTransaction(group[1].ID().String())
	key := f.keySnapshot()
	require.True(f.t, key.Registered)
	require.NotNil(f.t, key.Record)
	require.EqualValues(f.t, 1, key.Record.Epoch)
	require.Equal(f.t, f.owner[:], key.Record.PublicKey)
	require.Equal(f.t, balance+123, f.receiverBalance())
	snapshot, err := f.queries.Snapshot(context.Background(), f.asset, nil)
	require.NoError(f.t, err)
	require.Zero(f.t, snapshot.PrivateSupply)
	require.EqualValues(f.t, 100, snapshot.Total)
	f.checkPeerAndRestart(*pending.ConfirmedRound, key)
}
