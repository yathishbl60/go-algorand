//go:build algorand_native_privacy && algorand_native_privacy_stark && cgo && (linux || darwin)

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

package proofbridge

/*
#cgo CFLAGS: -I${SRCDIR}/../stark-ffi/include
#cgo LDFLAGS: ${SRCDIR}/../stark-ffi/lib/libalgorand_asset_privacy_ffi.a
#cgo linux LDFLAGS: -ldl -lpthread -lm
#cgo darwin LDFLAGS: -framework Security -framework CoreFoundation
#include "nap_privacy.h"
*/
import "C"

import (
	"runtime"
	"unsafe"
)

func backendStatus(implementation [64]byte) error {
	if len(backendArchiveDigest) != 128 {
		return ErrUnavailable
	}
	if C.nap_privacy_abi() != 1 {
		return ErrUnavailable
	}
	status := C.nap_privacy_source_v1((*C.uint8_t)(unsafe.Pointer(&implementation[0])), C.size_t(len(implementation)))
	runtime.KeepAlive(implementation)
	if status != 0 {
		return ErrUnavailable
	}
	return nil
}

func verifyBackend(request, bundle []byte) error {
	if len(request) == 0 || len(bundle) == 0 {
		return ErrRejected
	}
	status := C.nap_privacy_verify_v1(
		(*C.uint8_t)(unsafe.Pointer(&request[0])), C.size_t(len(request)),
		(*C.uint8_t)(unsafe.Pointer(&bundle[0])), C.size_t(len(bundle)))
	runtime.KeepAlive(request)
	runtime.KeepAlive(bundle)
	if status != 0 {
		return ErrRejected
	}
	return nil
}
