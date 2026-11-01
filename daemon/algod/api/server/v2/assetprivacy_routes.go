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

package v2

import "github.com/labstack/echo/v4"

type NativePrivacyRouter interface {
	GET(string, echo.HandlerFunc, ...echo.MiddlewareFunc) *echo.Route
}

// RegisterNativeAssetPrivacyHandlers uses the caller's ordinary API middleware.
func RegisterNativeAssetPrivacyHandlers(router NativePrivacyRouter, handler *Handlers, middleware ...echo.MiddlewareFunc) {
	router.GET("/v2/assets/:asset-id/privacy", handler.GetNativeAssetPrivacy, middleware...)
	router.GET("/v2/assets/:asset-id/privacy/outputs", handler.GetNativeAssetPrivacyOutputs, middleware...)
	router.GET("/v2/assets/:asset-id/privacy/notes/:note-index/witness", handler.GetNativeAssetPrivacyWitness, middleware...)
}
