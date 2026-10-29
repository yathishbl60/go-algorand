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

package verify

import (
	"errors"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
	"github.com/algorand/go-algorand/protocol"
)

var errNativePrivacyWitness = errors.New("invalid native privacy signed witness")
var errNativePrivacyConsent = errors.New("native privacy requires top-level post-quantum authorization")

// This is stateless admission only; normal PQ verification and ledger auth still run.
func checkNativePrivacySigned(tx *transactions.SignedTxn, proto config.ConsensusParams) (proofbridge.Limits, error) {
	if tx.Txn.Type != protocol.NativeAssetPrivacyTx {
		return proofbridge.Limits{}, errNativePrivacyWitness
	}
	if !proto.NativeAssetPrivacyExecutionConfigured() || !tx.NativeAssetPrivacySizeAllowed(proto) {
		return proofbridge.Limits{}, policy.ErrDisabled
	}
	suite, err := nativePrivacySignedSuite(tx)
	if err != nil {
		return proofbridge.Limits{}, err
	}
	selected, err := suitecatalog.Lookup(proto, suite)
	if err != nil {
		return proofbridge.Limits{}, err
	}
	return selected.Limits(), checkNativePrivacyConsent(tx, proto)
}

func checkNativePrivacyConsent(tx *transactions.SignedTxn, proto config.ConsensusParams) error {
	if len(tx.PQsig.Signature) == 0 || !tx.Authorizer().IsPQCompliant() ||
		!tx.Sig.Blank() || !tx.Msig.Blank() || !tx.Lsig.Blank() {
		return errNativePrivacyConsent
	}
	return tx.PQsig.ValidateEnvelope(proto, tx.Authorizer())
}

func nativePrivacyDiagnostic(tx transactions.SignedTxn) any {
	if tx.Txn.Type == protocol.NativeAssetPrivacyTx || len(tx.NativeAssetPrivacyProof) != 0 {
		return tx.ID()
	}
	return tx
}
