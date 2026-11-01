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

package query

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/accounting"
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func readPolicy(reader Reader, config Config, value *state) (err error) {
	value.Policy, err = readRecord(reader, config.Asset, policy.RecordKey, policy.DecodeRecord)
	return err
}

func readSupply(reader Reader, config Config, value *state) error {
	amount, err := readRecord(reader, config.Asset, accounting.SupplyKey, accounting.DecodePrivateSupply)
	if amount != nil {
		value.Private, value.supplyPresent = *amount, true
	}
	return err
}

func readRetirement(reader Reader, config Config, value *state) (err error) {
	value.Retired, err = readRecord(reader, config.Asset, retirement.Key, retirement.Decode)
	return err
}

func readClosure(reader Reader, config Config, value *state) (err error) {
	value.Closure, err = readRecord(reader, config.Asset, closure.Key, closure.Decode)
	return err
}

func readInventory(reader Reader, config Config, value *state) error {
	record, found, err := inventory.Read(reader, config.Asset)
	if found && err == nil {
		value.Inventory = &record
	}
	return err
}
