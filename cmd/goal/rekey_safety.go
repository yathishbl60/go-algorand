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
	"strings"

	"github.com/algorand/go-algorand/cmd/util/rekeysafety"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
)

type rekeyTxnInfo = rekeysafety.RekeyTxnInfo
type rekeyScanReport = rekeysafety.ScanReport

func parseRekeyWithSafety(rekeyToAddress string) basics.Address {
	rekeyTo := parseRekey(rekeyToAddress)
	ensureRekeyAllowed(rekeyTo)
	return rekeyTo
}

func ensureRekeyAllowed(rekeyTo basics.Address) {
	if rekeyTo == (basics.Address{}) || allowRekey {
		return
	}
	reportErrorf("Refusing to create a rekey transaction without --allow-rekey. Rekey target: %s", rekeyTo.String())
}

func ensureSafeToSign(stxns []transactions.SignedTxn, source string) {
	report := scanForRekey(stxns)
	if len(report.Rekeys) == 0 || allowRekey {
		return
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

	reportErrorf("Refusing to sign %s because it %s. Re-run with --allow-rekey only after verifying intent. %s", source, reason, strings.Join(details, "; "))
}

func scanForRekey(stxns []transactions.SignedTxn) rekeyScanReport {
	return rekeysafety.Scan(stxns)
}
