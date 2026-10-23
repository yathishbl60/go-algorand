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

// CreateBundle hashes the conventional evidence files and creates profile.json.
// It never overwrites a manifest or approves the evidence's security claims.
// Use a stable local directory; a write failure can leave a partial new manifest.
func CreateBundle(directory, profileID string) (string, error) {
	root, err := os.OpenRoot(directory)
	if err != nil {
		return "", fmt.Errorf("open bundle directory: %w", err)
	}
	defer root.Close()
	manifest, err := prepareManifest(root, profileID)
	if err != nil {
		return "", err
	}
	if err = writeManifest(root, manifest); err != nil {
		return "", err
	}
	return filepath.Join(directory, bundleManifest), nil
}

func prepareManifest(root *os.Root, profileID string) (Manifest, error) {
	manifest := Manifest{Version: manifestVersion, ProfileID: profileID, Artifacts: bundleLayout()}
	for i, artifact := range manifest.Artifacts {
		digest, err := digestArtifact(root, artifact.Path)
		if err != nil {
			return Manifest{}, err
		}
		manifest.Artifacts[i].SHA384 = digest
	}
	return manifest, validateManifest(manifest)
}
