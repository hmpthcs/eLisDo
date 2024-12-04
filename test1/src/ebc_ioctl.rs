use libc;
use std::{
    fs::OpenOptions,
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
};
use ioctl_readwrite_bad;

// #[repr(C)]
// pub struct payload {
//     trigger_global_refresh: bool,
// }

// number comes from a c printf(%lu, ....)
pub const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3221316672;
ioctl_readwrite_bad!(ebc_ioctl, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, libc::c_uchar);

pub fn trigger_global_refresh() {
    println!("Hello, world!");
    let ebc_device = "/dev/dri/by-path/platform-fdec0000.ebc-card";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(ebc_device).unwrap();

    let mut arg: u8 = 1;
    let arg_ptr: *mut u8 = &mut arg;
    unsafe{
        let result = ebc_ioctl(file.as_raw_fd(), arg_ptr);
        match result {
            Err(why) => panic!("{:?}", why),
            Ok(ret) => println!("{}", ret),
        }
    }
}
