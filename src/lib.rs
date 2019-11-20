#![no_std]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]

extern crate alloc;

use alloc::boxed::Box;
use linux_kernel_module::{self, println, cstr, CStr, bindings,
                          c_types, KernelModule, KernelResult};
use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags};

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
const BFS_MAX_LASTI: u64 = 513; // TODO: u64?

// Assertion: unsigned long == u64.
struct Inner {
    blocks: u64,
    freeb: u64,
    freei: u64,
    lf_eblk: u64,
    lasti: u64,
    // TODO:
    // DECLARE_BITMAP(si_imap, BFS_MAX_LASTI+1);
}

struct BFSSuperBlockInfo {
    // inner: Mutex<Inner>,
}

// End of code derived from bfs.h

impl FileSystem for BFS {
    const NAME: &'static CStr = cstr!("bfs-rust");
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;

    type SuperBlockInfo = BFSSuperBlockInfo;

    fn fill_super(fs_info: &mut Option<Box<Self::SuperBlockInfo>>) -> KernelResult<()> {
        *fs_info = Some(Box::new(BFSSuperBlockInfo {

        }));

        Ok(())
    }
}

impl KernelModule for BFSModule {
    fn init() -> KernelResult<Self> {
        println!("Hello kernel module!");

        let fs_registration = filesystem::register::<BFS>()?;

        Ok(BFSModule {
            _fs_registration: fs_registration,
        })
    }
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
