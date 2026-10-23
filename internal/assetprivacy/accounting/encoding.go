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

package accounting

import "encoding/binary"

const (
	supplyVersion   = 1
	supplyBytes     = 9
	supplyKeyPrefix = "aprv:s:"
)

// SupplyKey returns an asset-scoped KV key outside the AVM box namespace.
// This inactive development layout must be frozen in NAP-02 before activation.
func SupplyKey(assetID uint64) (string, error) {
	if assetID == 0 {
		return "", ErrAssetID
	}
	var suffix [8]byte
	binary.BigEndian.PutUint64(suffix[:], assetID)
	return supplyKeyPrefix + string(suffix[:]), nil
}

// EncodePrivateSupply stores only the public aggregate, not the fixed ASA total.
func EncodePrivateSupply(amount uint64) []byte {
	encoded := make([]byte, supplyBytes)
	encoded[0] = supplyVersion
	binary.BigEndian.PutUint64(encoded[1:], amount)
	return encoded
}

// DecodePrivateSupply rejects unknown versions and any noncanonical byte length.
func DecodePrivateSupply(encoded []byte) (uint64, error) {
	if len(encoded) != supplyBytes || encoded[0] != supplyVersion {
		return 0, ErrEncoding
	}
	return binary.BigEndian.Uint64(encoded[1:]), nil
}
