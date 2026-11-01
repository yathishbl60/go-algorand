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
	"net/http"
	"net/url"
)

func (client *NativePrivacyClient) get(ctx context.Context, path string, query url.Values, response any) error {
	if client == nil || client.http == nil || ctx == nil {
		return ErrNativePrivacyRequest
	}
	request, err := client.request(ctx, path, query)
	if err != nil {
		return err
	}
	result, err := client.http.Do(request)
	if err != nil {
		return nativePrivacyTransportError(err)
	}
	defer result.Body.Close()
	if result.StatusCode != http.StatusOK {
		return nativePrivacyHTTPError(result.StatusCode)
	}
	return nativePrivacyDecode(result.Body, response)
}

func (client *NativePrivacyClient) request(ctx context.Context, path string, query url.Values) (*http.Request, error) {
	endpoint := client.base.JoinPath(path)
	endpoint.RawQuery = query.Encode()
	request, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint.String(), nil)
	if err != nil {
		return nil, ErrNativePrivacyRequest
	}
	request.Header.Set(authHeader, client.token)
	request.Header.Set("Accept", "application/json")
	return request, nil
}
