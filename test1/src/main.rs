use input::{Libinput, LibinputInterface};
use libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::{fs::OpenOptionsExt, io::AsRawFd, io::OwnedFd};
use std::path::Path;
use std::thread;
use std::time::Duration;
//use nix::ioctl_readwrite_bad;

use input::event::keyboard::KeyboardEventTrait;

use input::event::tablet_tool::{TabletToolEventTrait, TabletToolProximityEvent};
use input::event::TabletToolEvent::Proximity;
use input::Event;
use input::Event::Tablet;

use input::event::touch::TouchEventTrait;
use input::event::TouchEvent;
use input::event::TouchEvent::{Down, Up};
use input::Event::Touch;

//mod ebc_ioctl;
mod sys_handler;

//use input::event::tablet_tool::TabletToolEventTrait;
//use input::tablet_tool::TabletToolProximityEvent;
//use input::event::tablet_tool::TabletTool;

//struct EbcObject {
//}
//
//
//const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3221316672;
//ioctl_readwrite_bad!(ebc_ioctl, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, libc::c_uchar);

//fn trigger_global_refresh() {
//    println!("Hello, world!");
//    let ebc_device = "/dev/dri/by-path/platform-fdec0000.ebc-card";
//    let file = OpenOptions::new()
//        .read(true)
//        .write(true)
//        .custom_flags(libc::O_NONBLOCK)
//        .open(ebc_device).unwrap();
//
//    let mut arg: u8 = 1;
//    let arg_ptr: *mut u8 = &mut arg;
//    unsafe{
//        let result = ebc_ioctl(file.as_raw_fd(), arg_ptr);
//       match result {
//            Err(why) => panic!("{:?}", why),
//            Ok(ret) => println!("{}", ret),
//        }
//    }
//}

struct Interface;

//const DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH: u64 = 3221316672;
//ioctl_readwrite_bad!(ebc_ioctl, DRM_IOCTL_ROCKCHIP_EBC_GLOBAL_REFRESH, libc::c_uchar);

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(/*(flags & O_RDONLY != 0) |*/ (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn handle_tablet_event(event: TabletToolProximityEvent) {
    let proximity = event.proximity_state();
    match proximity {
        input::event::tablet_tool::ProximityState::In => {
            //            println!("Proximity IN {:?}", proximity);
            sys_handler::set_bw_mode(2);
            sys_handler::set_default_waveform(1)
        }
        input::event::tablet_tool::ProximityState::Out => {
            //            println!("Proximity OUT {:?}", proximity);
            sys_handler::set_bw_mode(0);
            sys_handler::set_default_waveform(7)
        }
        _ => {} //    let down = proximity_state > 0.0;
    }
}

fn handle_touch_event(event: TouchEvent) {
    let touch = event.into_touch_event();
    match event {
        //            TouchEvent::Down(_) => {
        TouchEvent::Motion(_) => {
            // make sure we aren't being redundant && that there isn't a touchup event waiting to finish
            if touchmotion_action_inprogress == 0 && touchup_action_inprogress == 0 {
                println! {"new touch motion event"};
                touch_action_inprogress = 1; //set variable to reflect that we are in progress
                println! {"Current BW mode is {}", cur_bw_mode};
                println! {"Current default_waveform is {}", cur_default_wform}

                if cur_bw_mode != 2 {
                    //make sure we actually need to change bw mode first
                    sys_handler::set_bw_mode(2);
                    cur_bw_mode = 2;
                    println! {"New BW mode is {}", cur_bw_mode}
                }

                if cur_default_waveform != 2 {
                    sys_handler::set_default_waveform(2);
                    cur_default_waveform = 2;
                    println! {"New default waveform is {}", cur_default_waveform}
                }
                //set timer to wait until calling this completed
                thread::sleep(Duration::from_millis(100));
                touchmotion_action_in_progress = 0;
            }
        }

        TouchEvent::Up(_) => {
            //                TouchEvent::Frame(_) => {
            println!("Touch was UP");
            if touchup_action_inprogress == 0 {
                //make sure we aren't being redundant
                println! {"Current BW mode is {}", cur_bw_mode};
                println! {"Current default_waveform is {}", cur_default_wform};
                touchup_action_inprogress = 1;
                loop {
                    if touchmotion_action_inprogress = 1 {
                        // make sure touchmotion actions aren't in progress; if so, wait for touchmotion actions to complete
                        thread::sleep(Duration::from_millis(100));
                    }
                    if touchmotion_action_inprogress = 0 {
                        //                     thread::sleep(Duration::from_millis(200));
                        sys_handler::set_default_waveform(def_default_waveform);
                        thread::sleep(Duration::from_millis(100));
                        sys_handler::set_bw_mode(def_bw_mode);
                        touchup_action_inprogress = 0;
                        break; //exit the loop
                    }
                }
            }
        }
        // TODO: ESTABLISH SCREEN-CLEANING PROCEDURE--MAYBE FULLSCREEN REFRESH WITH NON-FLASHY OR FLASHY WAVEFORM--MAYBE TRY TO TRACK DAMAGE AREAS WITH WAYLAND/WLROOTS PROTOCOLS AND DO SOME WONKY STUFF TO TRIGGER FLASHY REFRESH ONLY ON DAMAGED AREAS..IDK
        //                    sys_handler::set_refresh_waveform(7);
        //                    use ebc_ioctl::trigger_global_refresh();
        //                    sys_handler::set_refresh_waveform(4)
        _ => {}
    }
}

fn main() {
    // Set some variables that we'll use to remain aware of where we are in the chain from event triggers to actions (avoid collisions / conflicts)
    let mut touchmotion_action_inprogress = 0;
    let mut touchup_action_inprogress = 0;
    let mut touch_action_complete = 0;
    //    let mut stylus_action_inprogress = 0;
    //    let mut stylus_action_complete = 0;

    // Read current driver parameters, use these as "defaults" for now
    let def_bw_mode = sys_handler::get_bw_mode();
    let def_default_wform = sys_handler::get_default_waveform();

    // Set up variables for current driver parameters
    let mut cur_bw_mode = def_bw_mode;
    let mut cur_default_wform = def_default_wform;

    // Do the libinput listening; todo: test alternative evdev listener
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();

        // Filter out the events we are watching for, trigger functions for each
        for event in &mut input {
            match event {
                Event::Tablet(Proximity(event)) => {
                    handle_tablet_event(event);
                }
                Event::Touch(event) => {
                    handle_touch_event(event);
                }
                _ => {}
            }
        }
        // Try to not hog cpu+battery; todo: optimize this more professionally
        std::thread::sleep(Duration::from_millis(5));
    }
}
// //            if let Tablet(Proximity(event)) = event {
// //             handle_tablet_event(event);
// //               }
//            if let Touch(event) = event {
//                handle_touch_event(event);
//                }
//            }
//        }
//    }
/*
fn handle_tablet_event(event:TabletToolProximityEvent){
    let proximity = event.proximity_state();
    match proximity {
         input::event::tablet_tool::ProximityState::In => {
            println!("Proximity IN {:?}", proximity);
        }
        input::event::tablet_tool::ProximityState::Out => {
            println!("Proximity OUT {:?}", proximity);
        }
        _ => {}

    //    let down = proximity_state > 0.0;
    }
}

fn handle_touch_event(event:TouchEvent) {
        match event {
            TouchEvent::Down(_) => {
                println!("Touch was DOWN");
                sys_handler::set_bw_mode(2);
                sys_handler::set_default_waveform(2)
            }
            TouchEvent::Up(_) => {
                println!("Touch was UP");
                sys_handler::set_bw_mode(0);
                sys_handler::set_default_waveform(7)
            }
            _ => {}
        }
    }
*/
//        println!("\nTouchDown {:?}", touch);

//            match event {
//                input::Event::Tablet(proximity) => {
//                    println!("\nProximity {} {:?}\n",
//                    &proximity.proximity(),
//                    proximity.Proximity_state()
//                    )
//                },
//                input::Event::Keyboard(key) => {
//                    println!("\nKey {} {:?}\n",
//                    &key.key(),
//                    key.key_state()
//                    )
//                },
//                _ => println!("wtf")
//            }
//        }
//    }
//}
