// Copyright 2021 Red Hat, Inc. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ffi::CStr;
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;

const EMPTY_CSTR: &[u8] = b"\0";

pub struct StatExt {
    pub st: libc::stat64,
    #[allow(dead_code)]
    pub mnt_id: u64,
}

pub fn stat64(f: &File) -> io::Result<StatExt> {
    let mut st = MaybeUninit::<libc::stat64>::zeroed();

    // Safe because this is a constant value and a valid C string.
    let pathname = unsafe { CStr::from_bytes_with_nul_unchecked(EMPTY_CSTR) };

    // Safe because the kernel will only write data in `st` and we check the return
    // value.
    let res = unsafe {
        libc::fstatat64(
            f.as_raw_fd(),
            pathname.as_ptr(),
            st.as_mut_ptr(),
            libc::AT_EMPTY_PATH | libc::AT_SYMLINK_NOFOLLOW,
        )
    };
    if res >= 0 {
        Ok(StatExt {
            // Safe because the kernel guarantees that the struct is now fully initialized.
            st: unsafe { st.assume_init() },
            mnt_id: 0,
        })
    } else {
        Err(io::Error::last_os_error())
    }
}
