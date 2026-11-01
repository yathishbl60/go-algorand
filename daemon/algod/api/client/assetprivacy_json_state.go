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

package client

import (
	"encoding/json"
	"strings"
)

func (state *nativePrivacyJSONState) step(token json.Token) error {
	delimiter, structural := token.(json.Delim)
	if structural && (delimiter == '}' || delimiter == ']') {
		if len(state.stack) == 0 {
			return ErrNativePrivacyResponse
		}
		state.stack = state.stack[:len(state.stack)-1]
		return nil
	}
	if err := state.element(token); err != nil {
		return err
	}
	if structural {
		if len(state.stack) >= 16 {
			return ErrNativePrivacyResponse
		}
		state.stack = append(state.stack, nativePrivacyJSONContainer{kind: delimiter, key: delimiter == '{'})
	}
	return nil
}

func (state *nativePrivacyJSONState) element(token json.Token) error {
	if len(state.stack) == 0 {
		if state.root || token != json.Delim('{') {
			return ErrNativePrivacyResponse
		}
		state.root = true
		return nil
	}
	current := &state.stack[len(state.stack)-1]
	if current.kind == '{' {
		return current.objectElement(token)
	}
	if current.kind == '[' {
		current.count++
		if current.count > 64 {
			return ErrNativePrivacyResponse
		}
	}
	return nil
}

func (current *nativePrivacyJSONContainer) objectElement(token json.Token) error {
	if !current.key {
		current.key = true
		return nil
	}
	name, ok := token.(string)
	if !ok {
		return ErrNativePrivacyResponse
	}
	name = strings.ToLower(name)
	if current.names == nil {
		current.names = make(map[string]bool)
	}
	if !current.uniqueName(name) {
		return ErrNativePrivacyResponse
	}
	current.names[name], current.key = true, false
	return nil
}

func (current *nativePrivacyJSONContainer) uniqueName(name string) bool {
	if len(current.names) >= 64 {
		return false
	}
	for previous := range current.names {
		if strings.EqualFold(previous, name) {
			return false
		}
	}
	return true
}
