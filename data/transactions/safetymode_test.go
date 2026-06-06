package transactions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/protocol"
	"github.com/algorand/go-algorand/test/partitiontest"
)

func TestComputeGroupOutcomeHashDeterministic(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	var sender basics.Address
	var receiver basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])

	tx := Transaction{Type: protocol.PaymentTx}
	tx.Sender = sender
	tx.Receiver = receiver
	tx.Amount = basics.MicroAlgos{Raw: 1234}
	tx.Fee = basics.MicroAlgos{Raw: 1000}

	group := []SignedTxnWithAD{{
		SignedTxn: SignedTxn{Txn: tx},
		ApplyData: ApplyData{ClosingAmount: basics.MicroAlgos{Raw: 50}},
	}}

	hash1 := ComputeGroupOutcomeHash(group)
	hash2 := ComputeGroupOutcomeHash(group)
	require.Equal(t, hash1, hash2)
}

func TestCollectGroupRiskSignals(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	var sender basics.Address
	var receiver basics.Address
	var closeTo basics.Address
	var rekeyTo basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])
	crypto.RandBytes(closeTo[:])
	crypto.RandBytes(rekeyTo[:])

	pay := Transaction{Type: protocol.PaymentTx}
	pay.Sender = sender
	pay.Receiver = receiver
	pay.RekeyTo = rekeyTo
	pay.CloseRemainderTo = closeTo

	appDel := Transaction{Type: protocol.ApplicationCallTx}
	appDel.Sender = sender
	appDel.ApplicationID = 77
	appDel.OnCompletion = DeleteApplicationOC

	group := []SignedTxnWithAD{
		{SignedTxn: SignedTxn{Txn: pay}, ApplyData: ApplyData{ClosingAmount: basics.MicroAlgos{Raw: 1}}},
		{SignedTxn: SignedTxn{Txn: appDel}},
	}

	signals := CollectGroupRiskSignals(group)
	require.Contains(t, signals, "rekeyChange")
	require.Contains(t, signals, "closeRemainderToUsed")
	require.Contains(t, signals, "appGlobalStateDelete")
}
