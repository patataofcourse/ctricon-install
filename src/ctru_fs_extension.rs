use std::{ffi::c_void, mem};

use ctru::services::fs::{Archive, ArchiveID, Fs, FsMediaType, PathType};

#[allow(dead_code)]
struct TotallyNotArchive {
    id: ArchiveID,
    handle: u64,
}

pub trait FsPlus {
    fn extdata(&self, id: u64, media_type: FsMediaType) -> ctru::Result<Archive>;
    fn binary_path<const T: usize>(&self, data: &[u32; T]) -> ctru_sys::FS_Path {
        ctru_sys::FS_Path {
            type_: PathType::Binary.into(),
            size: T as u32 * 4,
            data: data.as_ptr() as *const c_void,
        }
    }
}

impl FsPlus for Fs {
    fn extdata(&self, id: u64, media_type: FsMediaType) -> ctru::Result<Archive> {
        let id_lower = id as u32;
        let id_higher = (id >> 32) as u32;
        unsafe {
            let mut handle = 0;
            let id = ArchiveID::Extdata;
            let path = self.binary_path(&[media_type.into(), id_lower, id_higher]);
            let r = ctru_sys::FSUSER_OpenArchive(&mut handle, id.into(), path);
            if r < 0 {
                Err(ctru::Error::from(r))
            } else {
                Ok(mem::transmute(TotallyNotArchive { handle, id }))
            }
        }
    }
}
