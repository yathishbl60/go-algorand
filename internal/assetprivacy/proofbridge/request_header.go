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

package proofbridge

import "encoding/binary"

func (v *Verifier) writeHeader(encoded []byte, context Context, slots byte) {
	copy(encoded[:4], "NAPV")
	encoded[4] = 3
	copy(encoded[5:69], v.pin.Profile[:])
	copy(encoded[69:133], v.pin.Implementation[:])
	copy(encoded[133:197], v.pin.Digest[:])
	copy(encoded[197:229], context.Domain.Genesis[:])
	binary.BigEndian.PutUint64(encoded[229:237], context.Domain.Asset)
	copy(encoded[237:301], context.Domain.Policy[:])
	copy(encoded[301:365], context.Group[:])
	binary.BigEndian.PutUint32(encoded[365:369], v.limits.MaxBundleBytes)
	binary.BigEndian.PutUint32(encoded[369:373], v.limits.MaxProofBytes)
	binary.BigEndian.PutUint32(encoded[373:377], v.limits.MaxAllocationBytes)
	copy(encoded[377:1495], v.encoded[:])
	encoded[1495] = slots
}
