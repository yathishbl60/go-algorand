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
	"context"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
)

func (client *NativePrivacyClient) Outputs(ctx context.Context, asset basics.AssetIndex, options NativePrivacyOutputOptions) (model.NativeAssetPrivacyOutputPage, error) {
	if asset == 0 {
		return model.NativeAssetPrivacyOutputPage{}, ErrNativePrivacyRequest
	}
	values, expected, err := nativePrivacyOutputQuery(asset, options)
	if err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	var result model.NativeAssetPrivacyOutputPage
	if err = client.get(ctx, nativePrivacyPath(asset, "/outputs"), values, &result); err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	if err = nativePrivacyScope(asset, options.Round, result.Round, result.AssetId); err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	if err = expected.check(result); err != nil {
		return model.NativeAssetPrivacyOutputPage{}, err
	}
	return result, nil
}
