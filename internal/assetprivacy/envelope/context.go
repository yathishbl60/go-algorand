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

package envelope

import "encoding/binary"

// Context contains pre-encryption values; never include this envelope or group ID.
// Policy and Commitment are candidate SHA3-512 digests, Network is genesis hash.
type Context struct {
	Network    [32]byte
	Asset      uint64
	Policy     [64]byte
	Epoch      uint64
	Role       Role
	Nonce      [32]byte
	Output     uint32
	Commitment [64]byte
}

// Bytes returns a fixed-field, domain-separated associated-data encoding.
func (c Context) Bytes() ([]byte, error) {
	if err := c.validate(); err != nil {
		return nil, err
	}
	out := append([]byte("Algorand/AssetPrivacy/Candidate1/Envelope/"), formatVersion)
	out = append(out, c.Network[:]...)
	out = binary.BigEndian.AppendUint64(out, c.Asset)
	out = append(out, c.Policy[:]...)
	out = binary.BigEndian.AppendUint64(out, c.Epoch)
	out = append(out, byte(c.Role))
	out = append(out, c.Nonce[:]...)
	out = binary.BigEndian.AppendUint32(out, c.Output)
	return append(out, c.Commitment[:]...), nil
}

func (c Context) validate() error {
	if c.Asset == 0 || c.Epoch == 0 || c.Role < Recipient || c.Role > Controller {
		return ErrContext
	}
	if c.Network == [32]byte{} || c.Nonce == [32]byte{} {
		return ErrContext
	}
	if c.Policy == [64]byte{} || c.Commitment == [64]byte{} {
		return ErrContext
	}
	return nil
}
