// SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note
//
// This file is a derived work of the Linux kernel and of rust-bindgen;
// we have applied the kernel's license, as it is the more restrictive
// of the two, and as documentation has been copied verbatim from there.

//!  The raw ioctl interface defined by <linux/dm-ioctl.h>.
//!
//! Originally generated by rust-bindgen 0.69.5 from the <linux/dm-ioctl.h>
//! shipped with Linux 6.6.62, which identifies itself as API version
//! "4.48.0-ioctl (2023-03-01)", and then manually cleaned up.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use ::core::ffi::{c_char, c_int, c_uint, c_ulonglong};
use ::core::fmt;
use ::core::marker::PhantomData;

#[cfg(test)]
#[path = "tests/bindings.rs"]
mod tests;

#[repr(C)]
#[derive(Default)]
pub struct FlexibleArrayMember<T>(PhantomData<T>, [T; 0]);
impl<T> FlexibleArrayMember<T> {
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
}
impl<T> fmt::Debug for FlexibleArrayMember<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("FlexibleArrayMember")
    }
}

pub const DM_DIR: &[u8; 7] = b"mapper\0";
pub const DM_CONTROL_NODE: &[u8; 8] = b"control\0";

pub const DM_MAX_TYPE_NAME: u32 = 16;
pub const DM_NAME_LEN: usize = 128;
pub const DM_UUID_LEN: usize = 129;

/// Major version of the dm ioctl interface as defined by this header.
/// Not necessarily equal to the version as implemented by the running kernel.
pub const DM_VERSION_MAJOR: u32 = 4;

/// Minor version of the dm ioctl interface as defined by this header.
/// Not necessarily equal to the version as implemented by the running kernel.
pub const DM_VERSION_MINOR: u32 = 48;

/// Patchlevel version of the dm ioctl interface as defined by this header.
/// Not necessarily equal to the version as implemented by the running kernel.
pub const DM_VERSION_PATCHLEVEL: u32 = 0;

/// Extra version information.
pub const DM_VERSION_EXTRA: &[u8; 20] = b"-ioctl (2023-03-01)\0";

/// All ioctl arguments consist of a single chunk of memory,
/// with this structure at the start.  If a uuid is specified
/// any lookup (eg. for a DM_INFO) will be done on that, *not* the name.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dm_ioctl {
    /// The version number is made up of three parts:
    /// major - no backward or forward compatibility,
    /// minor - only backwards compatible,
    /// patch - both backwards and forwards compatible.
    ///
    /// All clients of the ioctl interface should fill in the
    /// version number of the interface that they were
    /// compiled with.
    ///
    /// All recognised ioctl commands (ie. those that don't
    /// return -ENOTTY) fill out this field, even if the
    /// command failed.
    pub version: [c_uint; 3usize],

    /// Total size of data passed in, including this struct.
    pub data_size: c_uint,

    /// Offset to start of the 'data' field, relative to the start of
    /// this struct.
    pub data_start: c_uint,

    /// ??? "in/out"
    pub target_count: c_uint,

    /// ??? "out"
    pub open_count: c_int,

    /// ??? "in/out"
    pub flags: c_uint,

    /// event_nr holds either the event number (input and output) or the
    /// uevent cookie value (input only).
    /// The DM_DEV_WAIT ioctl takes an event number as input.
    /// The DM_SUSPEND, DM_DEV_REMOVE and DM_DEV_RENAME ioctls
    /// use the field as a cookie to return in the DM_COOKIE
    /// variable with the uevents they issue.
    /// For output, the ioctls return the event number, not the cookie.
    pub event_nr: c_uint,

    /// Padding so that 'dev' is naturally aligned
    pub padding: c_uint,

    /// ??? "in/out"
    pub dev: c_ulonglong,

    /// Device name
    pub name: [c_char; DM_NAME_LEN],

    /// Unique identifier of the block device
    pub uuid: [c_char; DM_UUID_LEN],

    // ZW: FIXME: Should be FlexibleArrayMember<c_char> or even more precisely
    // union { data: FlexibleArrayMember<c_char>; pad: [c_char; 7] }
    // but then this struct could not be Copy.  If this is changed we
    // need to make sure that mem::size_of::<dm_ioctl>() still includes
    // these seven bytes so that _IOWR(DM_IOCTL, xxx, struct dm_ioctl)
    // values do not change.
    /// Padding or data
    pub data: [c_char; 7usize],
}

// ZW: Has to be impl'd by hand because there aren't built-in impls of
// Default for [c_char; 128] and [c_char; 129].  To be removed.
impl Default for dm_ioctl {
    fn default() -> Self {
        Self {
            version: [0, 0, 0],
            data_size: 0,
            data_start: 0,
            target_count: 0,
            open_count: 0,
            flags: 0,
            event_nr: 0,
            padding: 0,
            dev: 0,
            name: [0; DM_NAME_LEN],
            uuid: [0; DM_UUID_LEN],
            data: [0; 7],
        }
    }
}

/// Used to specify tables.  These structures appear after the dm_ioctl.
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_target_spec {
    /// ???
    pub sector_start: c_ulonglong,

    /// ???
    pub length: c_ulonglong,

    /// ??? "Used when reading from kernel only"
    pub status: c_int,

    /// Location of the next dm_target_spec.
    /// - When specifying targets on a DM_TABLE_LOAD command, this value is
    ///   the number of bytes from the start of the "current" dm_target_spec
    ///   to the start of the "next" dm_target_spec.
    /// - When retrieving targets on a DM_TABLE_STATUS command, this value
    ///   is the number of bytes from the start of the first dm_target_spec
    ///   (that follows the dm_ioctl struct) to the start of the "next"
    ///   dm_target_spec.
    pub next: c_uint,

    /// ???
    pub target_type: [c_char; 16usize],

    /// Parameter string starts immediately after this object.
    /// Be careful to add padding after string to ensure correct
    /// alignment of subsequent dm_target_spec.
    // note: not present in the kernel's struct definition
    pub params: FlexibleArrayMember<c_char>,
}

/// Used to retrieve the target dependencies.
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_target_deps {
    /// Array size
    pub count: c_uint,

    /// Padding so that 'dev' is naturally aligned; ignored
    pub padding: c_uint,

    /// ??? "out"
    pub dev: FlexibleArrayMember<c_ulonglong>,
}

/// Used to get a list of all dm devices.
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_name_list {
    /// ???
    pub dev: c_ulonglong,

    /// Offset to the next record from the _start_ of this one
    pub next: c_uint,

    /// ???
    pub name: FlexibleArrayMember<c_char>,
    /* The following members can be accessed by taking a pointer that
       points immediately after the terminating zero character in "name"
       and aligning this pointer to next 8-byte boundary.
       Uuid is present if the flag DM_NAME_LIST_FLAG_HAS_UUID is set.

       __u32 event_nr;
       __u32 flags;
       char uuid[0];

       [ZW: Flexible array member in the *middle* of a struct? Dude. WTF.]
    */
}

/// Used to retrieve the target versions
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_target_versions {
    /// ???
    pub next: c_uint,

    /// ???
    pub version: [c_uint; 3usize],

    /// ???
    pub name: FlexibleArrayMember<c_char>,
}

/// Used to pass message to a target
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_target_msg {
    /// ??? "Device sector"
    pub sector: c_ulonglong,

    /// ???
    pub message: FlexibleArrayMember<c_char>,
}
