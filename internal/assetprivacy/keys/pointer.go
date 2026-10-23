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

package keys

import (
	"bytes"
	"encoding/binary"
)

func (s *Store) pointer(epoch uint64) Write {
	encoded := append([]byte{1}, s.domain.bytes()...)
	encoded = binary.BigEndian.AppendUint64(encoded, epoch)
	return Write{s.domain.currentKey(), encoded}
}

func (s *Store) readPointer() (uint64, bool, error) {
	encoded, exists, err := s.kv.Get(s.domain.currentKey())
	if err != nil || !exists {
		return 0, exists, err
	}
	if len(encoded) != 146 || encoded[0] != 1 {
		return 0, true, ErrRecord
	}
	if !bytes.Equal(encoded[1:138], s.domain.bytes()) {
		return 0, true, ErrDomain
	}
	epoch := binary.BigEndian.Uint64(encoded[138:])
	if epoch == 0 {
		return 0, true, ErrEpoch
	}
	return epoch, true, nil
}
