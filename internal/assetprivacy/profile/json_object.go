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
	"encoding/json"
	"fmt"
)

func scanObject(decoder *json.Decoder, depth int) error {
	keys := make(map[string]bool)
	for decoder.More() {
		if err := readKey(decoder, keys); err != nil {
			return err
		}
		if err := scanValue(decoder, depth); err != nil {
			return err
		}
	}
	_, err := decoder.Token()
	return err
}

func readKey(decoder *json.Decoder, keys map[string]bool) error {
	token, err := decoder.Token()
	if err != nil {
		return err
	}
	key, ok := token.(string)
	if !ok {
		return fmt.Errorf("manifest object key must be a string")
	}
	if err := checkKey(key, keys); err != nil {
		return err
	}
	keys[key] = true
	return nil
}

func checkKey(key string, keys map[string]bool) error {
	switch key {
	case "version", "profile_id", "artifacts", "role", "path", "sha384":
	default:
		return fmt.Errorf("unknown manifest key %q", key)
	}
	if keys[key] {
		return fmt.Errorf("duplicate manifest key %q", key)
	}
	return nil
}
