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
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	basics_testing "github.com/algorand/go-algorand/data/basics/testing"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
	"github.com/stretchr/testify/require"
	"time"
)

func (f *ownerFixture) fundAccounts() {
	f.account = basics_testing.MakePQTestAccount(f.t, 51, protocol.PQSchemeFalcon1024)
	f.receiver = basics_testing.MakePQTestAccount(f.t, 52, protocol.PQSchemeFalcon1024).Address
	wallets, err := f.fixture.GetWalletsSortedByBalance()
	require.NoError(f.t, err)
	for _, target := range []string{f.account.Address.String(), f.receiver.String()} {
		tx, err := f.fixture.LibGoalClient.SendPaymentFromUnencryptedWallet(wallets[0].Address, target, 1000, 1_000_000_000, nil)
		require.NoError(f.t, err)
		f.waitTransaction(tx.ID().String())
	}
}

func (f *ownerFixture) sign(tx transactions.Transaction) transactions.SignedTxn {
	signature, err := f.account.Signer.Sign(tx)
	require.NoError(f.t, err)
	return transactions.SignedTxn{Txn: tx, PQsig: transactions.PQSig{Scheme: f.account.Scheme,
		Salt: f.account.Salt, PublicKey: f.account.PublicKey, Signature: signature}}
}

func (f *ownerFixture) fill(tx transactions.Transaction, fee uint64) transactions.Transaction {
	result, err := f.fixture.LibGoalClient.FillUnsignedTxTemplate(f.account.Address.String(), 0, 0, fee, tx)
	require.NoError(f.t, err)
	return result
}

func (f *ownerFixture) submit(tx transactions.Transaction) model.PendingTransactionResponse {
	signed := f.sign(tx)
	id, err := f.fixture.LibGoalClient.BroadcastTransaction(signed)
	require.NoError(f.t, err)
	return f.waitTransaction(id)
}

func (f *ownerFixture) waitTransaction(id string) model.PendingTransactionResponse {
	var pending model.PendingTransactionResponse
	var err error
	require.Eventually(f.t, func() bool {
		pending, err = f.fixture.LibGoalClient.PendingTransactionInformation(id)
		return err == nil && pending.ConfirmedRound != nil && *pending.ConfirmedRound > 0
	}, 90*time.Second, 250*time.Millisecond, "transaction must be accepted into a block")
	require.NoError(f.t, err)
	require.Empty(f.t, pending.PoolError)
	return pending
}
