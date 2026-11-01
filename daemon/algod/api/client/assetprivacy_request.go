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
	"fmt"
	"net/url"
	"strconv"

	"github.com/algorand/go-algorand/data/basics"
)

func nativePrivacyPath(asset basics.AssetIndex, suffix string) string {
	return fmt.Sprintf("v2/assets/%d/privacy%s", asset, suffix)
}

func nativePrivacyRoundQuery(round *basics.Round) url.Values {
	values := make(url.Values)
	if round != nil {
		values.Set("round", strconv.FormatUint(uint64(*round), 10))
	}
	return values
}

func nativePrivacyScope(asset basics.AssetIndex, requested *basics.Round, round, returned uint64) error {
	if asset == 0 {
		return ErrNativePrivacyRequest
	}
	if returned != uint64(asset) || requested != nil && round != uint64(*requested) {
		return ErrNativePrivacyResponse
	}
	return nil
}
