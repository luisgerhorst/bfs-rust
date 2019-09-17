#![no_std]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::{self, println, cstr, CStr};
use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags};

struct BFSModule {
    _fs_registration: filesystem::Registration<BFS>
}

// This is safe because Registration doesn't actually expose any methods. TODO: Sure?
unsafe impl Sync for BFSModule {}

struct BFS {}

impl FileSystem for BFS {
    // TODO: Is there a better way to do this?
    // https://github.com/gereeter/llvm-safe/blob/9710754e107db61965f257ed9d6c55e311dd32aa/examples/kaleidoscope_lib/trans.rs
    // uses a separate ConstCStr.
    const NAME: &'static CStr = unsafe { &*("bfs-rust\x00" as *const str as *const CStr) };
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;
}

impl linux_kernel_module::KernelModule for BFSModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");

        let fs_registration = filesystem::register::<BFS>()?;

        Ok(BFSModule {
            _fs_registration: fs_registration,
        })
    }
}

// extern "C" fn mount_callback<BFS>(
//     fs_type: *mut bindings::file_system_type,
//     flags: c_types::c_int,
//     dev_name: *const c_types::c_char,
//     data: *mut c_types::c_void,
// ) -> *mut bindings::dentry {
//     unsafe { bindings::mount_bdev(fs_type, flags, dev_name, Some(fill_super_callback::<BFS>)) }
// }

// extern "C" fn fill_super_callback<BFS>(
//     sb: *mut bindings::super_block,
//     data: *mut c_types::c_void,
//     silent: c_types::c_int,
// ) -> c_types::c_int {
//     // T::fill_super(...)
//     // This should actually create an object that gets dropped by
//     // file_system_registration::kill_sb. You can point to it with
//     // sb->s_fs_info.
//     unimplemented!();
// }

impl Drop for BFSModule {
    fn drop(&mut self) {
        println!("Goodbye kernel module!");
    }
}

linux_kernel_module::kernel_module!(
    BFSModule,
    author: "Fish in a Barrel Contributors",
    description: "An extremely simple kernel module",
    license: "GPL"
);
