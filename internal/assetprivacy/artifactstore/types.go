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

// Package artifactstore obtains bounded local registry data, never suite approval.
package artifactstore

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
)

var ErrUnavailable = errors.New("native privacy artifact is unavailable")

// Provider supplies public bytes identified by an independently selected digest.
// Callers must still validate consensus pins, layout and backend compatibility.
type Provider interface {
	Get(registryDigest [64]byte) ([artifactregistry.EncodedSize]byte, error)
}

type directory struct{ root string }

// NewDirectory remembers a trusted operator path without I/O or directory creation.
// An empty root remains unavailable rather than falling back to the working directory.
func NewDirectory(root string) Provider { return &directory{root: root} }
