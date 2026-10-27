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

// Package suitecatalog selects explicit consensus-pinned native privacy suites.
// Catalog validation and selection do not verify proofs or grant transaction authority.
package suitecatalog

import (
	"errors"

	"github.com/algorand/go-algorand/config"
)

// MaxSuites bounds complete catalog validation and snapshot storage.
const MaxSuites = 16

var (
	ErrUnknown = errors.New("native privacy suite is not in the current catalog")
	ErrCatalog = errors.New("invalid native privacy suite catalog")
)

// Suite is a value-only snapshot selected from a fully validated enabled catalog.
// Its zero value cannot construct a verifier. Selection is not lasting authorization.
type Suite struct {
	record config.NativeAssetPrivacySuite
}
