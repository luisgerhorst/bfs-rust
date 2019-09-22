#![no_std]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]

extern crate alloc;

use linux_kernel_module::{self, println, cstr, CStr};
use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags};
use linux_kernel_module::{bindings, c_types};

struct BFSModule {
    _fs_registration: filesystem::Registration<BFS>
}

struct BFS {}


// Start of code derived from bfs.h

// In theory BFS supports up to 512 inodes, numbered from 2 (for /) up to 513
// inclusive.  In actual fact, attempting to create the 512th inode (i.e. inode
// No. 513 or file No. 511) will fail with ENOSPC in bfs_add_entry(): the
// root directory cannot contain so many entries, counting '..'.  So,
// mkfs.bfs(8) should really limit its -N option to 511 and not 512. For now,
// we just print a warning if a filesystem is mounted with such "impossible
// to fill up" number of inodes
const BFS_MAX_LASTI = 513;

// Assertion: unsigned long == u64.
struct BFS_SB_INFO {
    u64 si_blocks;
    u64 si_freeb;
    u64 si_freei;
    u64 si_lf_eblk;
    u64 si_lasti;
    // TODO:
    //
    // DECLARE_BITMAP(si_imap, BFS_MAX_LASTI+1);
    // struct mutex bfs_lock;
    //
    // See https://github.com/torvalds/linux/blob/master/fs/bfs/bfs.h#L27
}

// End of code derived from bfs.h


impl FileSystem for BFS {
    const NAME: &'static CStr = cstr!("bfs-rust");
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

// TODO: Make this a method of FileSystem with a default implementation?
extern "C" fn mount_callback<BFS>(
    fs_type: *mut bindings::file_system_type,
    flags: c_types::c_int,
    dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut bindings::dentry {
    unsafe {
        bindings::mount_bdev(fs_type, flags, dev_name, data,
                             Some(fill_super_callback::<BFS>))
    }
}

extern "C" fn fill_super_callback<BFS>(
    _sb: *mut bindings::super_block,
    _data: *mut c_types::c_void,
    _silent: c_types::c_int,
) -> c_types::c_int {
    // T::fill_super(...)
    // This should actually create an object that gets dropped by
    // file_system_registration::kill_sb. You can point to it with
    // sb->s_fs_info.
    //
    // TODO: Is this really true? Isn't a fs_info destroyed when the filesystem
    // is unmounted?

    // TODO:
    //
    // 1. Set sb->s_fs_info to an in-memory structure representing the
    // superblock.
    // 2. s_time_min = 0, s_time_max = U32_MAX.
    // 3. ..., see https://github.com/torvalds/linux/blob/master/fs/bfs/inode.c#L330

    println!("Hello from fill_super_callback");
    unimplemented!();
}

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
