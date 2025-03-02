/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::Result;
use diced_open_dice::{derive_cdi_leaf_priv, sign, DiceArtifacts};
use diced_sample_inputs::make_sample_bcc_and_cdis;
use hwtrust::{dice, session::Session};

const EXPECTED_SAMPLE_CDI_ATTEST: &[u8] = &[
    0x3e, 0x57, 0x65, 0x5d, 0x48, 0x02, 0xbd, 0x5c, 0x66, 0xcc, 0x1f, 0x0f, 0xbe, 0x5e, 0x32, 0xb6,
    0x9e, 0x3d, 0x04, 0xaf, 0x00, 0x15, 0xbc, 0xdd, 0x1f, 0xbc, 0x59, 0xe4, 0xc3, 0x87, 0x95, 0x5e,
];

const EXPECTED_SAMPLE_CDI_SEAL: &[u8] = &[
    0x36, 0x1b, 0xd2, 0xb3, 0xc4, 0xda, 0x77, 0xb2, 0x9c, 0xba, 0x39, 0x53, 0x82, 0x93, 0xd9, 0xb8,
    0x9f, 0x73, 0x2d, 0x27, 0x06, 0x15, 0xa8, 0xcb, 0x6d, 0x1d, 0xf2, 0xb1, 0x54, 0xbb, 0x62, 0xf1,
];

const EXPECTED_SAMPLE_BCC: &[u8] = &[
    0x84, 0xa5, 0x01, 0x01, 0x03, 0x27, 0x04, 0x81, 0x02, 0x20, 0x06, 0x21, 0x58, 0x20, 0x3e, 0x85,
    0xe5, 0x72, 0x75, 0x55, 0xe5, 0x1e, 0xe7, 0xf3, 0x35, 0x94, 0x8e, 0xbb, 0xbd, 0x74, 0x1e, 0x1d,
    0xca, 0x49, 0x9c, 0x97, 0x39, 0x77, 0x06, 0xd3, 0xc8, 0x6e, 0x8b, 0xd7, 0x33, 0xf9, 0x84, 0x43,
    0xa1, 0x01, 0x27, 0xa0, 0x59, 0x01, 0x8a, 0xa9, 0x01, 0x78, 0x28, 0x34, 0x32, 0x64, 0x38, 0x38,
    0x36, 0x34, 0x66, 0x39, 0x37, 0x62, 0x36, 0x35, 0x34, 0x37, 0x61, 0x35, 0x30, 0x63, 0x31, 0x65,
    0x30, 0x61, 0x37, 0x34, 0x39, 0x66, 0x38, 0x65, 0x66, 0x38, 0x62, 0x38, 0x31, 0x65, 0x63, 0x36,
    0x32, 0x61, 0x66, 0x02, 0x78, 0x28, 0x31, 0x66, 0x36, 0x39, 0x36, 0x66, 0x30, 0x37, 0x32, 0x35,
    0x32, 0x66, 0x32, 0x39, 0x65, 0x39, 0x33, 0x66, 0x65, 0x34, 0x64, 0x65, 0x31, 0x39, 0x65, 0x65,
    0x33, 0x32, 0x63, 0x64, 0x38, 0x31, 0x64, 0x63, 0x34, 0x30, 0x34, 0x65, 0x37, 0x36, 0x3a, 0x00,
    0x47, 0x44, 0x50, 0x58, 0x40, 0x16, 0x48, 0xf2, 0x55, 0x53, 0x23, 0xdd, 0x15, 0x2e, 0x83, 0x38,
    0xc3, 0x64, 0x38, 0x63, 0x26, 0x0f, 0xcf, 0x5b, 0xd1, 0x3a, 0xd3, 0x40, 0x3e, 0x23, 0xf8, 0x34,
    0x4c, 0x6d, 0xa2, 0xbe, 0x25, 0x1c, 0xb0, 0x29, 0xe8, 0xc3, 0xfb, 0xb8, 0x80, 0xdc, 0xb1, 0xd2,
    0xb3, 0x91, 0x4d, 0xd3, 0xfb, 0x01, 0x0f, 0xe4, 0xe9, 0x46, 0xa2, 0xc0, 0x26, 0x57, 0x5a, 0xba,
    0x30, 0xf7, 0x15, 0x98, 0x14, 0x3a, 0x00, 0x47, 0x44, 0x53, 0x56, 0xa3, 0x3a, 0x00, 0x01, 0x11,
    0x71, 0x63, 0x41, 0x42, 0x4c, 0x3a, 0x00, 0x01, 0x11, 0x72, 0x01, 0x3a, 0x00, 0x01, 0x11, 0x73,
    0xf6, 0x3a, 0x00, 0x47, 0x44, 0x52, 0x58, 0x40, 0x47, 0xae, 0x42, 0x27, 0x4c, 0xcb, 0x65, 0x4d,
    0xee, 0x74, 0x2d, 0x05, 0x78, 0x2a, 0x08, 0x2a, 0xa5, 0xf0, 0xcf, 0xea, 0x3e, 0x60, 0xee, 0x97,
    0x11, 0x4b, 0x5b, 0xe6, 0x05, 0x0c, 0xe8, 0x90, 0xf5, 0x22, 0xc4, 0xc6, 0x67, 0x7a, 0x22, 0x27,
    0x17, 0xb3, 0x79, 0xcc, 0x37, 0x64, 0x5e, 0x19, 0x4f, 0x96, 0x37, 0x67, 0x3c, 0xd0, 0xc5, 0xed,
    0x0f, 0xdd, 0xe7, 0x2e, 0x4f, 0x70, 0x97, 0x30, 0x3a, 0x00, 0x47, 0x44, 0x54, 0x58, 0x40, 0xf9,
    0x00, 0x9d, 0xc2, 0x59, 0x09, 0xe0, 0xb6, 0x98, 0xbd, 0xe3, 0x97, 0x4a, 0xcb, 0x3c, 0xe7, 0x6b,
    0x24, 0xc3, 0xe4, 0x98, 0xdd, 0xa9, 0x6a, 0x41, 0x59, 0x15, 0xb1, 0x23, 0xe6, 0xc8, 0xdf, 0xfb,
    0x52, 0xb4, 0x52, 0xc1, 0xb9, 0x61, 0xdd, 0xbc, 0x5b, 0x37, 0x0e, 0x12, 0x12, 0xb2, 0xfd, 0xc1,
    0x09, 0xb0, 0xcf, 0x33, 0x81, 0x4c, 0xc6, 0x29, 0x1b, 0x99, 0xea, 0xae, 0xfd, 0xaa, 0x0d, 0x3a,
    0x00, 0x47, 0x44, 0x56, 0x41, 0x01, 0x3a, 0x00, 0x47, 0x44, 0x57, 0x58, 0x2d, 0xa5, 0x01, 0x01,
    0x03, 0x27, 0x04, 0x81, 0x02, 0x20, 0x06, 0x21, 0x58, 0x20, 0xb1, 0x02, 0xcc, 0x2c, 0xb2, 0x6a,
    0x3b, 0xe9, 0xc1, 0xd3, 0x95, 0x10, 0xa0, 0xe1, 0xff, 0x51, 0xde, 0x57, 0xd5, 0x65, 0x28, 0xfd,
    0x7f, 0xeb, 0xd4, 0xca, 0x15, 0xf3, 0xca, 0xdf, 0x37, 0x88, 0x3a, 0x00, 0x47, 0x44, 0x58, 0x41,
    0x20, 0x58, 0x40, 0x58, 0xd8, 0x03, 0x24, 0x53, 0x60, 0x57, 0xa9, 0x09, 0xfa, 0xab, 0xdc, 0x57,
    0x1e, 0xf0, 0xe5, 0x1e, 0x51, 0x6f, 0x9e, 0xa3, 0x42, 0xe6, 0x6a, 0x8c, 0xaa, 0xad, 0x08, 0x48,
    0xde, 0x7f, 0x4f, 0x6e, 0x2f, 0x7f, 0x39, 0x6c, 0xa1, 0xf8, 0x42, 0x71, 0xfe, 0x17, 0x3d, 0xca,
    0x31, 0x83, 0x92, 0xed, 0xbb, 0x40, 0xb8, 0x10, 0xe0, 0xf2, 0x5a, 0x99, 0x53, 0x38, 0x46, 0x33,
    0x97, 0x78, 0x05, 0x84, 0x43, 0xa1, 0x01, 0x27, 0xa0, 0x59, 0x01, 0x8a, 0xa9, 0x01, 0x78, 0x28,
    0x31, 0x66, 0x36, 0x39, 0x36, 0x66, 0x30, 0x37, 0x32, 0x35, 0x32, 0x66, 0x32, 0x39, 0x65, 0x39,
    0x33, 0x66, 0x65, 0x34, 0x64, 0x65, 0x31, 0x39, 0x65, 0x65, 0x33, 0x32, 0x63, 0x64, 0x38, 0x31,
    0x64, 0x63, 0x34, 0x30, 0x34, 0x65, 0x37, 0x36, 0x02, 0x78, 0x28, 0x32, 0x35, 0x39, 0x34, 0x38,
    0x39, 0x65, 0x36, 0x39, 0x37, 0x34, 0x38, 0x37, 0x30, 0x35, 0x64, 0x65, 0x33, 0x65, 0x32, 0x66,
    0x34, 0x34, 0x32, 0x36, 0x37, 0x65, 0x61, 0x34, 0x39, 0x33, 0x38, 0x66, 0x66, 0x36, 0x61, 0x35,
    0x37, 0x32, 0x35, 0x3a, 0x00, 0x47, 0x44, 0x50, 0x58, 0x40, 0xa4, 0x0c, 0xcb, 0xc1, 0xbf, 0xfa,
    0xcc, 0xfd, 0xeb, 0xf4, 0xfc, 0x43, 0x83, 0x7f, 0x46, 0x8d, 0xd8, 0xd8, 0x14, 0xc1, 0x96, 0x14,
    0x1f, 0x6e, 0xb3, 0xa0, 0xd9, 0x56, 0xb3, 0xbf, 0x2f, 0xfa, 0x88, 0x70, 0x11, 0x07, 0x39, 0xa4,
    0xd2, 0xa9, 0x6b, 0x18, 0x28, 0xe8, 0x29, 0x20, 0x49, 0x0f, 0xbb, 0x8d, 0x08, 0x8c, 0xc6, 0x54,
    0xe9, 0x71, 0xd2, 0x7e, 0xa4, 0xfe, 0x58, 0x7f, 0xd3, 0xc7, 0x3a, 0x00, 0x47, 0x44, 0x53, 0x56,
    0xa3, 0x3a, 0x00, 0x01, 0x11, 0x71, 0x63, 0x41, 0x56, 0x42, 0x3a, 0x00, 0x01, 0x11, 0x72, 0x01,
    0x3a, 0x00, 0x01, 0x11, 0x73, 0xf6, 0x3a, 0x00, 0x47, 0x44, 0x52, 0x58, 0x40, 0x93, 0x17, 0xe1,
    0x11, 0x27, 0x59, 0xd0, 0xef, 0x75, 0x0b, 0x2b, 0x1c, 0x0f, 0x5f, 0x52, 0xc3, 0x29, 0x23, 0xb5,
    0x2a, 0xe6, 0x12, 0x72, 0x6f, 0x39, 0x86, 0x65, 0x2d, 0xf2, 0xe4, 0xe7, 0xd0, 0xaf, 0x0e, 0xa7,
    0x99, 0x16, 0x89, 0x97, 0x21, 0xf7, 0xdc, 0x89, 0xdc, 0xde, 0xbb, 0x94, 0x88, 0x1f, 0xda, 0xe2,
    0xf3, 0xe0, 0x54, 0xf9, 0x0e, 0x29, 0xb1, 0xbd, 0xe1, 0x0c, 0x0b, 0xd7, 0xf6, 0x3a, 0x00, 0x47,
    0x44, 0x54, 0x58, 0x40, 0xb2, 0x69, 0x05, 0x48, 0x56, 0xb5, 0xfa, 0x55, 0x6f, 0xac, 0x56, 0xd9,
    0x02, 0x35, 0x2b, 0xaa, 0x4c, 0xba, 0x28, 0xdd, 0x82, 0x3a, 0x86, 0xf5, 0xd4, 0xc2, 0xf1, 0xf9,
    0x35, 0x7d, 0xe4, 0x43, 0x13, 0xbf, 0xfe, 0xd3, 0x36, 0xd8, 0x1c, 0x12, 0x78, 0x5c, 0x9c, 0x3e,
    0xf6, 0x66, 0xef, 0xab, 0x3d, 0x0f, 0x89, 0xa4, 0x6f, 0xc9, 0x72, 0xee, 0x73, 0x43, 0x02, 0x8a,
    0xef, 0xbc, 0x05, 0x98, 0x3a, 0x00, 0x47, 0x44, 0x56, 0x41, 0x01, 0x3a, 0x00, 0x47, 0x44, 0x57,
    0x58, 0x2d, 0xa5, 0x01, 0x01, 0x03, 0x27, 0x04, 0x81, 0x02, 0x20, 0x06, 0x21, 0x58, 0x20, 0x96,
    0x6d, 0x96, 0x42, 0xda, 0x64, 0x51, 0xad, 0xfa, 0x00, 0xbc, 0xbc, 0x95, 0x8a, 0xb0, 0xb9, 0x76,
    0x01, 0xe6, 0xbd, 0xc0, 0x26, 0x79, 0x26, 0xfc, 0x0f, 0x1d, 0x87, 0x65, 0xf1, 0xf3, 0x99, 0x3a,
    0x00, 0x47, 0x44, 0x58, 0x41, 0x20, 0x58, 0x40, 0x10, 0x7f, 0x77, 0xad, 0x70, 0xbd, 0x52, 0x81,
    0x28, 0x8d, 0x24, 0x81, 0xb4, 0x3f, 0x21, 0x68, 0x9f, 0xc3, 0x80, 0x68, 0x86, 0x55, 0xfb, 0x2e,
    0x6d, 0x96, 0xe1, 0xe1, 0xb7, 0x28, 0x8d, 0x63, 0x85, 0xba, 0x2a, 0x01, 0x33, 0x87, 0x60, 0x63,
    0xbb, 0x16, 0x3f, 0x2f, 0x3d, 0xf4, 0x2d, 0x48, 0x5b, 0x87, 0xed, 0xda, 0x34, 0xeb, 0x9c, 0x4d,
    0x14, 0xac, 0x65, 0xf4, 0xfa, 0xef, 0x45, 0x0b, 0x84, 0x43, 0xa1, 0x01, 0x27, 0xa0, 0x59, 0x01,
    0x8f, 0xa9, 0x01, 0x78, 0x28, 0x32, 0x35, 0x39, 0x34, 0x38, 0x39, 0x65, 0x36, 0x39, 0x37, 0x34,
    0x38, 0x37, 0x30, 0x35, 0x64, 0x65, 0x33, 0x65, 0x32, 0x66, 0x34, 0x34, 0x32, 0x36, 0x37, 0x65,
    0x61, 0x34, 0x39, 0x33, 0x38, 0x66, 0x66, 0x36, 0x61, 0x35, 0x37, 0x32, 0x35, 0x02, 0x78, 0x28,
    0x35, 0x64, 0x34, 0x65, 0x64, 0x37, 0x66, 0x34, 0x31, 0x37, 0x61, 0x39, 0x35, 0x34, 0x61, 0x31,
    0x38, 0x31, 0x34, 0x30, 0x37, 0x62, 0x35, 0x38, 0x38, 0x35, 0x61, 0x66, 0x64, 0x37, 0x32, 0x61,
    0x35, 0x62, 0x66, 0x34, 0x30, 0x64, 0x61, 0x36, 0x3a, 0x00, 0x47, 0x44, 0x50, 0x58, 0x40, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a,
    0x00, 0x47, 0x44, 0x53, 0x58, 0x1a, 0xa3, 0x3a, 0x00, 0x01, 0x11, 0x71, 0x67, 0x41, 0x6e, 0x64,
    0x72, 0x6f, 0x69, 0x64, 0x3a, 0x00, 0x01, 0x11, 0x72, 0x0c, 0x3a, 0x00, 0x01, 0x11, 0x73, 0xf6,
    0x3a, 0x00, 0x47, 0x44, 0x52, 0x58, 0x40, 0x26, 0x1a, 0xbd, 0x26, 0xd8, 0x37, 0x8f, 0x4a, 0xf2,
    0x9e, 0x49, 0x4d, 0x93, 0x23, 0xc4, 0x6e, 0x02, 0xda, 0xe0, 0x00, 0x02, 0xe7, 0xed, 0x29, 0xdf,
    0x2b, 0xb3, 0x69, 0xf3, 0x55, 0x0e, 0x4c, 0x22, 0xdc, 0xcf, 0xf5, 0x92, 0xc9, 0xfa, 0x78, 0x98,
    0xf1, 0x0e, 0x55, 0x5f, 0xf4, 0x45, 0xed, 0xc0, 0x0a, 0x72, 0x2a, 0x7a, 0x3a, 0xd2, 0xb1, 0xf7,
    0x76, 0xfe, 0x2a, 0x6b, 0x7b, 0x2a, 0x53, 0x3a, 0x00, 0x47, 0x44, 0x54, 0x58, 0x40, 0x04, 0x25,
    0x5d, 0x60, 0x5f, 0x5c, 0x45, 0x0d, 0xf2, 0x9a, 0x6e, 0x99, 0x30, 0x03, 0xb8, 0xd6, 0xe1, 0x99,
    0x71, 0x1b, 0xf8, 0x44, 0xfa, 0xb5, 0x31, 0x79, 0x1c, 0x37, 0x68, 0x4e, 0x1d, 0xc0, 0x24, 0x74,
    0x68, 0xf8, 0x80, 0x20, 0x3e, 0x44, 0xb1, 0x43, 0xd2, 0x9c, 0xfc, 0x12, 0x9e, 0x77, 0x0a, 0xde,
    0x29, 0x24, 0xff, 0x2e, 0xfa, 0xc7, 0x10, 0xd5, 0x73, 0xd4, 0xc6, 0xdf, 0x62, 0x9f, 0x3a, 0x00,
    0x47, 0x44, 0x56, 0x41, 0x01, 0x3a, 0x00, 0x47, 0x44, 0x57, 0x58, 0x2d, 0xa5, 0x01, 0x01, 0x03,
    0x27, 0x04, 0x81, 0x02, 0x20, 0x06, 0x21, 0x58, 0x20, 0xdb, 0xe7, 0x5b, 0x3f, 0xa3, 0x42, 0xb0,
    0x9c, 0xf8, 0x40, 0x8c, 0xb0, 0x9c, 0xf0, 0x0a, 0xaf, 0xdf, 0x6f, 0xe5, 0x09, 0x21, 0x11, 0x92,
    0xe1, 0xf8, 0xc5, 0x09, 0x02, 0x3d, 0x1f, 0xb7, 0xc5, 0x3a, 0x00, 0x47, 0x44, 0x58, 0x41, 0x20,
    0x58, 0x40, 0xc4, 0xc1, 0xd7, 0x1c, 0x2d, 0x26, 0x89, 0x22, 0xcf, 0xa6, 0x99, 0x77, 0x30, 0x84,
    0x86, 0x27, 0x59, 0x8f, 0xd8, 0x08, 0x75, 0xe0, 0xb2, 0xef, 0xf9, 0xfa, 0xa5, 0x40, 0x8c, 0xd3,
    0xeb, 0xbb, 0xda, 0xf2, 0xc8, 0xae, 0x41, 0x22, 0x50, 0x9c, 0xe8, 0xb2, 0x9c, 0x9b, 0x3f, 0x8a,
    0x78, 0x76, 0xab, 0xd0, 0xbe, 0xfc, 0xe4, 0x79, 0xcb, 0x1b, 0x2b, 0xaa, 0x4d, 0xdd, 0x15, 0x61,
    0x42, 0x06,
];
const MESSAGE: &[u8] = b"Message for testing";

#[test]
fn sample_bcc_and_cdis_are_as_expected() {
    let dice_artifacts = make_sample_bcc_and_cdis().unwrap();
    assert_eq!(dice_artifacts.cdi_attest(), EXPECTED_SAMPLE_CDI_ATTEST);
    assert_eq!(dice_artifacts.cdi_seal(), EXPECTED_SAMPLE_CDI_SEAL);
    assert_eq!(dice_artifacts.bcc(), Some(EXPECTED_SAMPLE_BCC));
}

#[test]
fn cdi_leaf_priv_corresponds_to_leaf_public_key_in_dice_chain() -> Result<()> {
    let dice_artifacts = make_sample_bcc_and_cdis().unwrap();
    let private_key = derive_cdi_leaf_priv(&dice_artifacts).unwrap();
    let signature = sign(MESSAGE, private_key.as_array()).unwrap();

    let session = Session::default();
    let chain = dice::Chain::from_cbor(&session, dice_artifacts.bcc().unwrap())?;
    let public_key = chain.leaf().subject_public_key();
    public_key.verify(&signature, MESSAGE)
}
