/* Copyright (C) 2026 Algorand Foundation Ltd.
 * SPDX-License-Identifier: AGPL-3.0-or-later */
#ifndef ALGORAND_NATIVE_PRIVACY_H
#define ALGORAND_NATIVE_PRIVACY_H

#include <stddef.h>
#include <stdint.h>

/* ABI 1. Trusted caller owns/pins immutable ranges until return; no pointer escapes.
 * request is native context/configuration, never a peer-supplied witness request.
 * 0=all proofs checked, 1=pointer/size, 2=artifact, 3=rejected, 4=backend panic.
 * A zero result grants no transaction authority and performs no state writes. */
uint32_t nap_privacy_abi(void);
/* 0=expected compiled source, 1=invalid range/length, 2=source mismatch, 4=panic. */
uint32_t nap_privacy_source_v1(const uint8_t *source, size_t source_len);
uint32_t nap_privacy_verify_v1(const uint8_t *request, size_t request_len,
                             const uint8_t *bundle, size_t bundle_len);

#endif
