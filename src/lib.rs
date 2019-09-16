#![no_std]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::{self, println, cstr, CStr};
use linux_kernel_module::filesystem::{register, FileSystem, FileSystemFlags};

struct HelloWorldModule {
    message: String,
}

struct BFS {}

impl FileSystem for BFS {
    // TODO: Is there a better way to do this?
    // https://github.com/gereeter/llvm-safe/blob/9710754e107db61965f257ed9d6c55e311dd32aa/examples/kaleidoscope_lib/trans.rs
    // uses a separate ConsCStr.
    const NAME: &'static CStr = unsafe { &*("bfs-rust\x00" as *const str as *const CStr) };

    // TODO: Which flags are the default?
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");

        // let mut fs_registration_kernel_result = register::<BFS>();
        Ok(HelloWorldModule {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Goodbye kernel module!");
    }
}

linux_kernel_module::kernel_module!(
    HelloWorldModule,
    author: "Fish in a Barrel Contributors",
    description: "An extremely simple kernel module",
    license: "GPL"
);
