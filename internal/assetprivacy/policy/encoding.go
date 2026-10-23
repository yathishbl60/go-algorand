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

package policy

import "encoding/binary"

// RecordKey reserves native policy state outside AVM box names.
func RecordKey(asset uint64) (string, error) {
	if asset == 0 {
		return "", ErrRecord
	}
	return string(binary.BigEndian.AppendUint64([]byte("aprv:p:"), asset)), nil
}

// EncodeRecord returns a canonical, versioned 77-byte native record.
func EncodeRecord(r Record) ([]byte, error) {
	if err := r.Validate(); err != nil {
		return nil, err
	}
	out := []byte{1, byte(r.Policy.Modes), byte(r.InitialCapabilities), byte(r.Policy.Capabilities), byte(r.Policy.Disclosures)}
	out = append(out, r.Suite[:]...)
	return binary.BigEndian.AppendUint64(out, r.Created), nil
}

func DecodeRecord(encoded []byte) (Record, error) {
	if len(encoded) != 77 || encoded[0] != 1 {
		return Record{}, ErrRecord
	}
	r := Record{InitialCapabilities: Capabilities(encoded[2]), Created: binary.BigEndian.Uint64(encoded[69:77])}
	r.Policy = Policy{Modes: Modes(encoded[1]), Capabilities: Capabilities(encoded[3]), Disclosures: Disclosures(encoded[4])}
	copy(r.Suite[:], encoded[5:69])
	if err := r.Validate(); err != nil {
		return Record{}, err
	}
	return r, nil
}
