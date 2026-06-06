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

package main

import (
	"fmt"
	"io"
	"strings"

	"github.com/spf13/cobra"

	"github.com/algorand/go-algorand/cmd/util/rekeysafety"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

var allowRekey bool

type rekeyTxnInfo = rekeysafety.RekeyTxnInfo
type rekeyScanReport = rekeysafety.ScanReport

func addAllowRekeyFlag(cmd *cobra.Command) {
	cmd.Flags().BoolVar(&allowRekey, "allow-rekey", false, "Acknowledge and allow signing a transaction that changes an account's spending authority")
}

func decodeSignedTxns(data []byte) ([]transactions.SignedTxn, error) {
	var txns []transactions.SignedTxn
	dec := protocol.NewMsgpDecoderBytes(data)
	for {
		var stxn transactions.SignedTxn
		err := dec.Decode(&stxn)
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, err
		}
		txns = append(txns, stxn)
	}
	return txns, nil
}

func validateSafeToSign(stxns []transactions.SignedTxn, source string) error {
	report := scanForRekey(stxns)
	if len(report.Rekeys) == 0 || allowRekey {
		return nil
	}

	var details []string
	for _, txn := range report.Rekeys {
		detail := fmt.Sprintf("txn[%d] %s sender=%s rekey-to=%s", txn.Index, txn.Type, txn.Sender.String(), txn.RekeyTo.String())
		if txn.AssetOptIn {
			detail += " asset-opt-in"
		}
		details = append(details, detail)
	}

	reason := "contains rekeyed transactions"
	if report.HasAssetOptInAndRekey {
		reason = "contains an asset opt-in grouped with rekeying"
	}

	return fmt.Errorf("refusing to sign %s because it %s. Re-run with --allow-rekey only after verifying intent. %s", source, reason, strings.Join(details, "; "))
}

func scanForRekey(stxns []transactions.SignedTxn) rekeyScanReport {
	return rekeysafety.Scan(stxns)
}
