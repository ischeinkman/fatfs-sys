#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[cfg(feature="create-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature="create-bindings"))]
mod bindings;
#[cfg(not(feature="create-bindings"))]
pub use bindings::*;

#[repr(C)]
pub enum DRESULT {
    RES_OK = 0,     /* 0: Successful */
    RES_ERROR,      /* 1: R/W Error */
    RES_WRPRT,      /* 2: Write Protected */
    RES_NOTRDY,     /* 3: Not Ready */
    RES_PARERR      /* 4: Invalid Parameter */
}

use DRESULT::*;

pub type DSTATUS = u8;

/// User provided fatfs methods.
pub trait FatfsDiskHandler : Send {
    fn disk_status(&mut self, _pdrv: BYTE) -> DSTATUS { 0 as DSTATUS }

    fn disk_initialize(&mut self, pdrv: BYTE) -> DSTATUS { disk_status(pdrv) }

    fn disk_read(&mut self, pdrv: BYTE, buf: *mut BYTE, sector: DWORD, count: UINT) -> DRESULT;

    fn disk_write(&mut self, pdrv: BYTE, buf: *const BYTE, sector: DWORD, count: UINT) -> DRESULT;

    fn disk_ioctl(&mut self, _pdrv: BYTE, _cmd: BYTE, _buf: *mut libc::c_void) -> DRESULT { RES_PARERR as DRESULT }
}

lazy_static::lazy_static! {
    static ref DISK_HANDLER: std::sync::Mutex<Option<Box<dyn FatfsDiskHandler>>> = std::sync::Mutex::new(None);
}

/// Register user-provided fatfs functions. All fatfs functions will panic if this is not called.
pub unsafe fn register_disk_handler(handler: impl FatfsDiskHandler + 'static) {
    *DISK_HANDLER.lock().unwrap() = Some(Box::new(handler));
}

#[no_mangle]
pub extern fn disk_status(pdrv: BYTE) -> DSTATUS { DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_status(pdrv) }

#[no_mangle]
pub extern fn disk_initialize(pdrv: BYTE) -> DSTATUS { DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_initialize(pdrv) }

#[no_mangle]
pub extern fn disk_read(pdrv: BYTE, buf: *mut BYTE, sector: DWORD, count: UINT) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_read(pdrv, buf, sector, count)
}

#[no_mangle]
pub extern fn disk_write(pdrv: BYTE, buf: *const BYTE, sector: DWORD, count: UINT) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_write(pdrv, buf, sector, count)
}

#[no_mangle]
pub extern fn disk_ioctl(pdrv: BYTE, cmd: BYTE, buf: *mut libc::c_void) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_ioctl(pdrv, cmd, buf)
}
/* Disk Status Bits (DSTATUS) */

pub const STA_NOINIT : DSTATUS = 0x01; /* Drive not : DSTATUS = initialized*/
pub const STA_NODISK : DSTATUS = 0x02; /* No medium in the : DSTATUS = drive*/
pub const STA_PROTECT : DSTATUS = 0x04; /* Write : DSTATUS = protected*/


/* Command code for disk_ioctrl fucntion */

/* Generic command (Used by : DSTATUS = FatFs)*/
pub const CTRL_SYNC : BYTE = 0; /* Complete pending write process (needed at FF_FS_READONLY == : DSTATUS = 0)*/
pub const GET_SECTOR_COUNT : BYTE = 1; /* Get media size (needed at FF_USE_MKFS == : DSTATUS = 1)*/
pub const GET_SECTOR_SIZE : BYTE = 2; /* Get sector size (needed at FF_MAX_SS != : DSTATUS = FF_MIN_SS)*/
pub const GET_BLOCK_SIZE : BYTE = 3; /* Get erase block size (needed at FF_USE_MKFS == : DSTATUS = 1)*/
pub const CTRL_TRIM : BYTE = 4; /* Inform device that the data on the block of sectors is no longer used (needed at FF_USE_TRIM == : DSTATUS = 1)*/