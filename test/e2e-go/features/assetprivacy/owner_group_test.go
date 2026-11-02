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
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/protocol"
	"github.com/stretchr/testify/require"
)

func (f *ownerFixture) registrationGroup() ([]transactions.SignedTxn, []byte) {
	bodies := f.registrationBodies()
	groupID, err := f.fixture.LibGoalClient.GroupID(bodies)
	require.NoError(f.t, err)
	for i := range bodies {
		bodies[i].Group = groupID
	}
	binding, err := statement.Bind(f.network, f.suite.ID, 1, bodies, statement.Limits{
		MaxGroupSize: 16, MaxBodyEstimate: 5_000_000, MaxGroupEstimate: 5_000_000})
	require.NoError(f.t, err)
	valid, invalid := f.proofWitnesses(binding)
	group := []transactions.SignedTxn{f.sign(bodies[0]), f.sign(bodies[1])}
	group[1].NativeAssetPrivacyProof = valid
	f.measureRegistrationWire(group)
	return group, invalid
}

func (f *ownerFixture) registrationBodies() []transactions.Transaction {
	payment := transactions.Transaction{Type: protocol.PaymentTx, PaymentTxnFields: transactions.PaymentTxnFields{
		Receiver: f.receiver, Amount: basics.MicroAlgos{Raw: 123}}}
	register := lifecyclebody.Body{Operation: lifecyclebody.Register, Asset: uint64(f.asset), Policy: f.policy,
		Suite: f.suite.ID, Role: keys.RecipientSpend, Account: [32]byte(f.account.Address), PublicKey: f.owner[:]}
	return []transactions.Transaction{f.fill(payment, 100_000), f.lifecycleTransaction(register)}
}
