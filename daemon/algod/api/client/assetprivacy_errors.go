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
	"errors"
	"net/http"
	"net/url"
	"strconv"
)

// NativePrivacyTransportError preserves cancellation without logging request URLs.
type NativePrivacyTransportError struct{ cause error }

func (NativePrivacyTransportError) Error() string     { return "native privacy request transport failed" }
func (err NativePrivacyTransportError) Unwrap() error { return err.cause }

func nativePrivacyTransportError(err error) error {
	var request *url.Error
	if errors.As(err, &request) {
		err = request.Err
	}
	return NativePrivacyTransportError{cause: err}
}

// Never include untrusted response bodies, credentials or request URLs in errors.
func nativePrivacyHTTPError(status int) error {
	return HTTPError{StatusCode: status, Status: strconv.Itoa(status) + " " + http.StatusText(status),
		ErrorString: "native privacy query rejected"}
}
