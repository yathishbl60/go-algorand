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
	"fmt"
	"os"
	"path/filepath"
)

// VerifyBundle checks bounded local files against a manifest beside those files.
// Checksums establish consistency only; the manifest itself is not authenticated.
func VerifyBundle(path string) (Report, error) {
	root, err := os.OpenRoot(filepath.Dir(path))
	if err != nil {
		return Report{}, fmt.Errorf("open bundle directory: %w", err)
	}
	defer root.Close()
	manifest, err := loadManifest(root, filepath.Base(path))
	if err != nil {
		return Report{}, err
	}
	if err = verifyManifest(root, manifest); err != nil {
		return Report{}, err
	}
	return Report{manifest.ProfileID, len(manifest.Artifacts),
		"artifact integrity only; security approval and protocol activation are not assessed"}, nil
}

func verifyManifest(root *os.Root, manifest Manifest) error {
	if err := validateManifest(manifest); err != nil {
		return err
	}
	for _, artifact := range manifest.Artifacts {
		if err := verifyArtifact(root, artifact); err != nil {
			return err
		}
	}
	return nil
}
