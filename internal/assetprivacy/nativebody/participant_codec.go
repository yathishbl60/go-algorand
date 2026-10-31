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

package nativebody

func (p *parsed) readParticipants(encoded []byte) error {
	if len(encoded) != p.header.participantSize() {
		return ErrEncoding
	}
	if len(encoded) != 0 {
		for i := range p.header.Participants {
			copy(p.header.Participants[i][:], encoded[i*32:(i+1)*32])
		}
	}
	return p.header.validateParticipants()
}

func (b Body) finishEncoding(encoded []byte, at int) []byte {
	if b.participantSize() != 0 {
		for i, account := range b.Participants {
			copy(encoded[at+i*32:at+(i+1)*32], account[:])
		}
	}
	return encoded
}
