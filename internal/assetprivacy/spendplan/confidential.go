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

package spendplan

const (
	ConfidentialTransfer             Kind = 8
	ConfidentialUnshield             Kind = 9
	ConfidentialTransferPublicValues      = 392
	ConfidentialUnshieldPublicValues      = 324
)

func (k Kind) transfer() bool     { return k == Transfer || k == ConfidentialTransfer }
func (k Kind) unshield() bool     { return k == Unshield || k == ConfidentialUnshield }
func (k Kind) confidential() bool { return k == ConfidentialTransfer || k == ConfidentialUnshield }
func (k Kind) valid() bool        { return k.transfer() || k.unshield() }

func (s Statement) checkParticipants() error {
	if !s.Kind.confidential() {
		if s.Participants != [4][32]byte{} {
			return ErrStatement
		}
		return nil
	}
	for i, account := range s.Participants {
		active := i < 2 || s.Kind.transfer()
		if (account != [32]byte{}) != active {
			return ErrStatement
		}
	}
	return nil
}

func appendParticipants(values []uint16, statement Statement) []uint16 {
	if !statement.Kind.confidential() {
		return values
	}
	for _, account := range statement.Participants {
		for _, value := range account {
			values = append(values, uint16(value))
		}
	}
	return values
}

func (s Statement) checkFields() error {
	if err := s.checkParticipants(); err != nil {
		return err
	}
	return s.checkOutputs()
}
