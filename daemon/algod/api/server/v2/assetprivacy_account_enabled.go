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

package v2

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
)

// Keep the obligation, ordinary account data and consensus prices at one round.
// A pruned/unavailable round is an error, never a latest-round fallback.
func nativeAccountMinimumBalance(lgr LedgerForAPI, round basics.Round, addr basics.Address, base uint64, proto config.ConsensusParams) (uint64, error) {
	if !proto.EnableNativeAssetPrivacy || addr.IsZero() {
		return base, nil
	}
	extra, err := nativeAccountStorageMinimum(lgr, round, addr, proto)
	if err != nil {
		return 0, err
	}
	minimum, overflow := basics.OAdd(base, extra)
	if overflow {
		return 0, storage.ErrArithmetic
	}
	return minimum, nil
}

func nativeAccountStorageMinimum(lgr LedgerForAPI, round basics.Round, addr basics.Address, proto config.ConsensusParams) (uint64, error) {
	key, err := storage.Key(addr)
	if err != nil {
		return 0, err
	}
	encoded, err := lgr.LookupKv(round, key)
	if err != nil || encoded == nil {
		return 0, err
	}
	obligation, err := storage.Decode(encoded)
	if err != nil {
		return 0, err
	}
	return obligation.Required(proto.NativeAssetPrivacyFlatMinBalance, proto.NativeAssetPrivacyByteMinBalance)
}
