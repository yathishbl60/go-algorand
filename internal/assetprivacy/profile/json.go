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

package profile

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
)

func checkJSON(data []byte) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	if err := scanValue(decoder, 0); err != nil {
		return err
	}
	if _, err := decoder.Token(); err != io.EOF {
		return errors.New("manifest must contain exactly one JSON value")
	}
	return nil
}

func scanValue(decoder *json.Decoder, depth int) error {
	if depth > 8 {
		return errors.New("manifest JSON nesting exceeds limit")
	}
	token, err := decoder.Token()
	if err != nil {
		return err
	}
	switch token {
	case json.Delim('{'):
		return scanObject(decoder, depth+1)
	case json.Delim('['):
		return scanArray(decoder, depth+1)
	default:
		return nil
	}
}

func scanArray(decoder *json.Decoder, depth int) error {
	for decoder.More() {
		if err := scanValue(decoder, depth); err != nil {
			return err
		}
	}
	_, err := decoder.Token()
	return err
}
