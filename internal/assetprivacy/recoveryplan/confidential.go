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

package recoveryplan

const (
	ConfidentialPartialPrivate Kind = 11 + iota
	ConfidentialFullPrivate
	ConfidentialPartialPublic
	ConfidentialFullPublic
)

const (
	ConfidentialPartialPrivatePublicValues = 320
	ConfidentialFullPrivatePublicValues    = 288
	ConfidentialPartialPublicPublicValues  = 292
	ConfidentialFullPublicPublicValues     = 260
)

func (k Kind) Confidential() bool {
	return k >= ConfidentialPartialPrivate && k <= ConfidentialFullPublic
}

func (k Kind) Public() bool { return k.public() }

func (k Kind) base() Kind {
	if k.Confidential() {
		return k - 10
	}
	return k
}

func (p *Plan) Participants() [4][32]byte { return p.participants }

func (s Statement) checkParticipants() error {
	if !s.Kind.Confidential() {
		if s.Participants != [4][32]byte{} {
			return ErrStatement
		}
		return nil
	}
	for i, account := range s.Participants {
		active := i == 0 || i >= 2 && i-2 < s.Kind.outputCount()
		if (account != [32]byte{}) != active {
			return ErrStatement
		}
	}
	return s.checkRemainderParticipant()
}

func appendParticipants(values []uint16, statement Statement) []uint16 {
	if !statement.Kind.Confidential() {
		return values
	}
	for _, account := range statement.Participants {
		for _, value := range account {
			values = append(values, uint16(value))
		}
	}
	return values
}
