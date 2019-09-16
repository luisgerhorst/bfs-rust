#![no_std]
#![feature(const_str_as_bytes)]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::{self, println, cstr, CStr};
use linux_kernel_module::filesystem::{register, FileSystem, FileSystemFlags};

struct HelloWorldModule {
    message: String,
}

// struct BFS {}

// impl FileSystem for BFS {
//     // TODO: https://github.com/gereeter/llvm-safe/blob/9710754e107db61965f257ed9d6c55e311dd32aa/examples/kaleidoscope_lib/trans.rs
//     const NAME: &'static CStr = const_cstr!("bfsrs");
//     // TODO: Which flags are the default?
//     const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;
// }

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
