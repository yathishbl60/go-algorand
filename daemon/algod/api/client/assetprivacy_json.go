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
	"bytes"
	"encoding/json"
	"io"
)

type nativePrivacyJSONContainer struct {
	kind  json.Delim
	count int
	key   bool
	names map[string]bool
}
type nativePrivacyJSONState struct {
	stack []nativePrivacyJSONContainer
	root  bool
}

// Bound collections before decoding generated models with dynamic slices.
func nativePrivacyJSONShape(raw []byte) error {
	decoder := json.NewDecoder(bytes.NewReader(raw))
	state := nativePrivacyJSONState{}
	for tokens := 0; tokens < 8192; tokens++ {
		token, err := decoder.Token()
		if err == io.EOF && state.root && len(state.stack) == 0 {
			return nil
		}
		if err != nil {
			return ErrNativePrivacyResponse
		}
		if err = state.step(token); err != nil {
			return err
		}
	}
	return ErrNativePrivacyResponse
}
