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

type parsed struct {
	header      Body
	inputs      [2]Input
	outputs     [2]Output
	inputCount  int
	outputCount int
}

// Decode validates the complete bounded encoding before allocating owned payloads.
// Keep encoded immutable until this call returns; no input buffer is retained.
func Decode(encoded []byte) (Body, error) {
	parsed, err := preflight(encoded)
	if err != nil {
		return Body{}, err
	}
	return parsed.own(), nil
}

func preflight(encoded []byte) (parsed, error) {
	if len(encoded) < HeaderSize || len(encoded) > MaxSize || string(encoded[:5]) != "NAPT\x01" {
		return parsed{}, ErrEncoding
	}
	value := parsed{header: readHeader(encoded)}
	if err := value.header.validateHeader(); err != nil {
		return parsed{}, err
	}
	value.inputCount, value.outputCount, _ = value.header.Operation.arity()
	if int(encoded[223]) != value.inputCount || int(encoded[224]) != value.outputCount {
		return parsed{}, ErrArity
	}
	if err := value.readRecords(encoded); err != nil {
		return parsed{}, err
	}
	return value, nil
}

func (p parsed) own() Body {
	body := p.header
	body.Inputs = append([]Input(nil), p.inputs[:p.inputCount]...)
	body.Outputs = make([]Output, p.outputCount)
	for index := range body.Outputs {
		body.Outputs[index] = p.outputs[index].clone()
	}
	return body
}
