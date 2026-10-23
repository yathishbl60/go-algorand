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

package profile

import (
	"crypto/sha512"
	"encoding/hex"
	"fmt"
	"io"
	"os"
)

func digestArtifact(root *os.Root, name string) (string, error) {
	file, err := openRegular(root, name, maxArtifactBytes)
	if err != nil {
		return "", err
	}
	defer file.Close()
	digest := sha512.New384()
	size, err := io.Copy(digest, io.LimitReader(file, maxArtifactBytes+1))
	if err != nil {
		return "", fmt.Errorf("hash %q: %w", name, err)
	}
	if size == 0 || size > maxArtifactBytes {
		return "", fmt.Errorf("artifact %q must contain 1-%d bytes", name, maxArtifactBytes)
	}
	return hex.EncodeToString(digest.Sum(nil)), nil
}
