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

package artifactregistry

import "bytes"

func (pin Pin) validate() error {
	if pin.Profile == [64]byte{} || pin.Implementation == [64]byte{} || pin.Digest == [64]byte{} {
		return ErrPin
	}
	return nil
}

func checkInput(encoded []byte, pin Pin) error {
	if string(encoded[:4]) != "NAPK" || encoded[4] != 4 || encoded[133] != EntryCount {
		return ErrEncoding
	}
	if !bytes.Equal(encoded[5:69], pin.Profile[:]) || !bytes.Equal(encoded[69:133], pin.Implementation[:]) {
		return ErrPin
	}
	if sum(encoded) != pin.Digest {
		return ErrPin
	}
	return nil
}
