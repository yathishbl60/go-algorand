package eval

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
	"github.com/algorand/go-algorand/test/partitiontest"
)

func TestValidateGroupSafetyAttestationSuccess(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	var sender basics.Address
	var receiver basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])

	tx := transactions.Transaction{Type: protocol.PaymentTx}
	tx.Sender = sender
	tx.Receiver = receiver
	tx.Amount = basics.MicroAlgos{Raw: 10}
	tx.Fee = basics.MicroAlgos{Raw: 1000}

	stxn := transactions.SignedTxn{Txn: tx}
	apply := transactions.ApplyData{}

	txgroup := []transactions.SignedTxnWithAD{{SignedTxn: stxn, ApplyData: apply}}
	hash := transactions.ComputeGroupOutcomeHash(txgroup)
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	txgroup[0].SignedTxn.Txn.ExpectedOutcomeHash = hash

	txibs := []transactions.SignedTxnInBlock{{SignedTxnWithAD: transactions.SignedTxnWithAD{SignedTxn: txgroup[0].SignedTxn, ApplyData: apply}}}

	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.NoError(t, err)
}

func TestValidateGroupSafetyAttestationMismatch(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	var sender basics.Address
	var receiver basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])

	tx := transactions.Transaction{Type: protocol.PaymentTx}
	tx.Sender = sender
	tx.Receiver = receiver
	tx.Amount = basics.MicroAlgos{Raw: 10}
	tx.Fee = basics.MicroAlgos{Raw: 1000}

	stxn := transactions.SignedTxn{Txn: tx}
	apply := transactions.ApplyData{}

	txgroup := []transactions.SignedTxnWithAD{{SignedTxn: stxn, ApplyData: apply}}
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	txgroup[0].SignedTxn.Txn.ExpectedOutcomeHash = crypto.Hash([]byte("not-the-right-hash"))

	txibs := []transactions.SignedTxnInBlock{{SignedTxnWithAD: transactions.SignedTxnWithAD{SignedTxn: txgroup[0].SignedTxn, ApplyData: apply}}}

	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "mismatch")
}

func TestValidateGroupSafetyAttestationRiskSignalMode(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	var sender basics.Address
	var receiver basics.Address
	var rekeyTo basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])
	crypto.RandBytes(rekeyTo[:])

	tx := transactions.Transaction{Type: protocol.PaymentTx}
	tx.Sender = sender
	tx.Receiver = receiver
	tx.RekeyTo = rekeyTo
	tx.Fee = basics.MicroAlgos{Raw: 1000}

	stxn := transactions.SignedTxn{Txn: tx}
	txgroup := []transactions.SignedTxnWithAD{{SignedTxn: stxn, ApplyData: transactions.ApplyData{}}}
	hash := transactions.ComputeGroupOutcomeHash(txgroup)
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashAndRiskEnforced
	txgroup[0].SignedTxn.Txn.ExpectedOutcomeHash = hash

	txibs := []transactions.SignedTxnInBlock{{SignedTxnWithAD: transactions.SignedTxnWithAD{SignedTxn: txgroup[0].SignedTxn, ApplyData: transactions.ApplyData{}}}}

	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "risk signals")
}

func TestValidateGroupSafetyAttestationInvalidMode(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	txgroup, txibs := makeSingleTxGroup(t)
	txgroup[0].SignedTxn.Txn.SafetyMode = uint8(255)
	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "invalid safety mode")
}

func TestValidateGroupSafetyAttestationInconsistentMode(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	txgroup, txibs := makeTwoTxGroup(t)
	hash := transactions.ComputeGroupOutcomeHash(txgroup)
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	txgroup[0].SignedTxn.Txn.ExpectedOutcomeHash = hash
	txgroup[1].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashAndRiskEnforced
	txgroup[1].SignedTxn.Txn.ExpectedOutcomeHash = hash

	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "inconsistent safety mode")
}

func TestValidateGroupSafetyAttestationInconsistentExpectedHash(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	txgroup, txibs := makeTwoTxGroup(t)
	hash := transactions.ComputeGroupOutcomeHash(txgroup)
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	txgroup[1].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	txgroup[0].SignedTxn.Txn.ExpectedOutcomeHash = hash
	txgroup[1].SignedTxn.Txn.ExpectedOutcomeHash = crypto.Hash([]byte("other"))

	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "inconsistent expected outcome hash")
}

func TestValidateGroupSafetyAttestationEmptyExpectedHash(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	txgroup, txibs := makeSingleTxGroup(t)
	txgroup[0].SignedTxn.Txn.SafetyMode = transactions.SafetyModeHashEnforced
	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.Error(t, err)
	require.Contains(t, err.Error(), "expected outcome hash is empty")
}

func TestValidateGroupSafetyAttestationDisabledModeNoop(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	txgroup, txibs := makeSingleTxGroup(t)
	err := validateGroupSafetyAttestation(txgroup, txibs)
	require.NoError(t, err)
}

func makeSingleTxGroup(t *testing.T) ([]transactions.SignedTxnWithAD, []transactions.SignedTxnInBlock) {
	t.Helper()
	stxn := signedPaymentTxn()
	txad := transactions.SignedTxnWithAD{SignedTxn: stxn, ApplyData: transactions.ApplyData{}}
	txib := transactions.SignedTxnInBlock{SignedTxnWithAD: txad}
	return []transactions.SignedTxnWithAD{txad}, []transactions.SignedTxnInBlock{txib}
}

func makeTwoTxGroup(t *testing.T) ([]transactions.SignedTxnWithAD, []transactions.SignedTxnInBlock) {
	t.Helper()
	oneGroup, oneBlock := makeSingleTxGroup(t)
	twoGroup, twoBlock := makeSingleTxGroup(t)
	group := []transactions.SignedTxnWithAD{oneGroup[0], twoGroup[0]}
	blocks := []transactions.SignedTxnInBlock{oneBlock[0], twoBlock[0]}
	return group, blocks
}

func signedPaymentTxn() transactions.SignedTxn {
	var sender, receiver basics.Address
	crypto.RandBytes(sender[:])
	crypto.RandBytes(receiver[:])
	tx := transactions.Transaction{Type: protocol.PaymentTx}
	tx.Sender = sender
	tx.Receiver = receiver
	tx.Amount = basics.MicroAlgos{Raw: 10}
	tx.Fee = basics.MicroAlgos{Raw: 1000}
	return transactions.SignedTxn{Txn: tx}
}
