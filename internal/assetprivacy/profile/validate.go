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
	"encoding/hex"
	"fmt"
	"io/fs"
	"strings"
)

func validateManifest(manifest Manifest) error {
	if manifest.Version != manifestVersion {
		return fmt.Errorf("unsupported manifest version %d", manifest.Version)
	}
	id := manifest.ProfileID
	if strings.TrimSpace(id) == "" || id != strings.TrimSpace(id) || len(id) > 128 {
		return fmt.Errorf("profile_id must be 1-128 bytes without surrounding whitespace")
	}
	return validateArtifacts(manifest.Artifacts)
}

func validateArtifacts(artifacts []Artifact) error {
	roles := requiredRoles()
	if len(artifacts) != len(roles) {
		return fmt.Errorf("manifest requires exactly %d evidence artifacts", len(roles))
	}
	paths := make(map[string]bool)
	for _, artifact := range artifacts {
		if err := validateArtifact(artifact); err != nil {
			return err
		}
		if !roles[artifact.Role] || paths[artifact.Path] {
			return fmt.Errorf("unknown/duplicate role or duplicate path for %q", artifact.Role)
		}
		delete(roles, artifact.Role)
		paths[artifact.Path] = true
	}
	return nil
}

func validateArtifact(artifact Artifact) error {
	if !fs.ValidPath(artifact.Path) || artifact.Path == "." || strings.Contains(artifact.Path, "\\") {
		return fmt.Errorf("artifact %q requires a relative slash-separated path", artifact.Role)
	}
	digest, err := hex.DecodeString(artifact.SHA384)
	if err != nil || len(digest) != 48 {
		return fmt.Errorf("artifact %q requires a 96-character SHA-384 hex digest", artifact.Role)
	}
	return nil
}
