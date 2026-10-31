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

package retirement

import "errors"

const EncodedSize = 177

var ErrRecord = errors.New("invalid native privacy retirement")
var ErrRetired = errors.New("native privacy asset is retired")
var ErrActive = errors.New("native privacy asset is not retired")
var ErrEpoch = errors.New("stale native privacy cleanup epoch")

type Record struct {
	Manager        [32]byte
	Policy, Suite  [64]byte
	Created, Epoch uint64
}

func (r Record) Validate() error {
	if r.Manager == [32]byte{} || r.Policy == [64]byte{} || r.Suite == [64]byte{} || r.Epoch == 0 {
		return ErrRecord
	}
	return nil
}
