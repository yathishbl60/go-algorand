//go:build algorand_native_privacy && (linux || darwin)

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

package artifactstore

import (
	"errors"
	"os"

	"golang.org/x/sys/unix"
)

// Nonblocking open and an atomic no-follow leaf check avoid FIFO/symlink races.
func openRegular(path string) (*os.File, error) {
	fd, err := unix.Open(path, unix.O_RDONLY|unix.O_NONBLOCK|unix.O_NOFOLLOW|unix.O_CLOEXEC, 0)
	if err != nil {
		return nil, err
	}
	file := os.NewFile(uintptr(fd), path)
	if file == nil {
		unix.Close(fd)
		return nil, errors.New("cannot own registry file descriptor")
	}
	info, err := file.Stat()
	if err != nil || !info.Mode().IsRegular() {
		file.Close()
		return nil, errors.New("registry artifact must be a regular file")
	}
	return file, nil
}
