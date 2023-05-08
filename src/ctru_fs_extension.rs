use std::{ffi::c_void, mem};

use ctru::services::fs::{ArchiveID, FsMediaType, Archive, PathType, Fs};


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
            println!("{:?} {:X?}", path, *(path.data as *const [u32; 3]));
            let r = ctru_sys::FSUSER_OpenArchive(&mut handle, id.into(), path);
            if r < 0 {
                Err(ctru::Error::from(r))
            } else {
                Ok(mem::transmute(TotallyNotArchive { handle, id }))
            }
        }
    }
}

pub trait ArchivePlus {
    fn check_file(&self, path: impl Into<String>) -> ctru::Result<()>;
    fn read_file(&self, path: String) -> ctru::Result<Vec<u8>>;
    fn write_file(&self, path: String, contents: &[u8]) -> ctru::Result<()>;
}

impl ArchivePlus for Archive {
    fn check_file(&self, path: impl Into<String>) -> ctru::Result<()> {
        let archive: TotallyNotArchive = unsafe { mem::transmute_copy(self) };
        let mut handle = 0;

        let res = unsafe {
            ctru_sys::FSUSER_OpenFile(
                &mut handle,
                archive.handle,
                ctru_sys::fsMakePath(
                    ctru_sys::PATH_ASCII,
                    (path.into() + "\0").as_ptr() as *const c_void,
                ),
                ctru_sys::FS_OPEN_READ,
                0,
            )
        };
        if res != 0 {
            Err(ctru::Error::Os(res))?
        } 

        let res = unsafe {
            ctru_sys::FSFILE_Close(handle)
        };
        if res != 0 {
            Err(ctru::Error::Os(res))
        } else {
            Ok(())
        }
    }

    fn read_file(&self, path: String) -> ctru::Result<Vec<u8>> {
        let archive: TotallyNotArchive = unsafe { mem::transmute_copy(self) };
        let mut handle = 0;

        let res = unsafe {
            ctru_sys::FSUSER_OpenFile(
                &mut handle,
                archive.handle,
                ctru_sys::fsMakePath(
                    ctru_sys::PATH_ASCII,
                    (path + "\0").as_ptr() as *const c_void,
                ),
                ctru_sys::FS_OPEN_READ,
                0,
            )
        };
        if res != 0 {
            Err(ctru::Error::Os(res))?
        }

        let size = unsafe {
            let mut size = 0;
            let res = ctru_sys::FSFILE_GetSize(handle, &mut size);
            if res != 0 {
                Err(ctru::Error::Os(res))?
            }
            size
        };

        let mut contents = vec![0u8; size as usize];

        let res = unsafe {
            let mut null = 0;
            ctru_sys::FSFILE_Read(
                handle,
                &mut null,
                0,
                contents.as_mut_ptr() as *mut c_void,
                size as u32,
            )
        };
        if res != 0 {
            Err(ctru::Error::Os(res))?
        }

        let res = unsafe {
            ctru_sys::FSFILE_Close(handle)
        };
        if res != 0 {
            Err(ctru::Error::Os(res))
        } else {
            Ok(contents)
        }
    }

    fn write_file(&self, path: String, contents: &[u8]) -> ctru::Result<()> {
        todo!();
    }
}
