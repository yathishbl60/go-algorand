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
	"io"
)

const nativePrivacyResponseBytes = 1 << 20

func nativePrivacyDecode(body io.Reader, response any) error {
	raw, err := io.ReadAll(io.LimitReader(body, nativePrivacyResponseBytes+1))
	if err != nil {
		return nativePrivacyTransportError(err)
	}
	if len(raw) > nativePrivacyResponseBytes {
		return ErrNativePrivacyResponse
	}
	if err = nativePrivacyJSONShape(raw); err != nil {
		return err
	}
	if err = json.Unmarshal(raw, response); err != nil {
		return ErrNativePrivacyResponse
	}
	return nil
}
