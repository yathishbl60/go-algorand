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
	"net/http"
	"time"
)

// NativePrivacyQueries clones HTTP settings; redirects are always disabled.
func (client RestClient) NativePrivacyQueries(httpClient *http.Client) (*NativePrivacyClient, error) {
	base := client.serverURL
	if base.Host == "" || base.Scheme != "http" && base.Scheme != "https" || base.User != nil ||
		base.RawQuery != "" || base.Fragment != "" || base.Opaque != "" {
		return nil, ErrNativePrivacyRequest
	}
	configured := http.Client{Timeout: 30 * time.Second}
	if httpClient != nil {
		configured = *httpClient
	}
	if configured.Timeout <= 0 {
		configured.Timeout = 30 * time.Second
	}
	configured.CheckRedirect = nativePrivacyNoRedirect
	return &NativePrivacyClient{base: base, token: client.apiToken, http: &configured}, nil
}

func nativePrivacyNoRedirect(*http.Request, []*http.Request) error { return http.ErrUseLastResponse }
