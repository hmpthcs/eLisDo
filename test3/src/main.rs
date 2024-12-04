// This file will serve as a prototype for the part of the program that receives notifications
// about and/or listens for hardware events, waiting until it see's one we've configured it to
// care about.
// It's also the "handler" of these notifications, which sends them off to the "effector" part
// of the program, where we do stuff in response per our configuration
//
//
//
//

use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use input::event::{tablet_tool::TabletToolProximityEvent, TabletToolEvent::Proximity};
use input::event::{EventTrait, TouchEvent};
use input::Event;
//use std::any::Any;
//use std::hash::Hash;
//use std::rc::Rc;
use std::thread;
use std::time::Duration;

use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

// mod ebc_ioctl;
mod sys_handler;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();

    //enum Touchstate  { touchstate
    //
    //}

    // pub i32 touchstate = 0;
    //  println!("initial touchstate: {:?}", touchstate);

    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            //      println!("Got event: {:?}", event);
            use input::event::TabletToolEvent::*;
            use input::Event::*;

            match event {
                //                Event::Tablet(Proximity(event)) => {
                Event::Tablet(Proximity(ev)) => {
                    handle_tablet_event(ev);
                }
                Touch(ev) => {
                    handle_touch_event(ev);
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(500));
        //      println!("current touchstate is: {:?}", touchstate)
    }
}

//
// HANDLERS BELOW
//

fn handle_tablet_event(ev: TabletToolProximityEvent) {
    let proximity = ev.proximity_state();

    println!("...proximity...");

    match proximity {
        input::event::tablet_tool::ProximityState::In => {
            println!("...{:?}", proximity);

            // Todo: set ebc_pwr on here
            //        sys_handler::set_panel_pwr_control(on);

            sys_handler::set_bw_mode(1); // b,w
            sys_handler::set_refresh_waveform(2); // 16->b,w

            // Todo: fs refresh here
            //            ebc_ioctl::trigger_global_refresh;

            sys_handler::set_default_waveform(1)
        }

        input::event::tablet_tool::ProximityState::Out => {
            println!("...{:?}", proximity);
        } //        _ => {} //    let down = proximity_state > 0.0;
    }
}

fn handle_touch_event(ev: TouchEvent) {
    //    println!("TOUCH event = {:?}", event);
    match ev {
        TouchEvent::Down(_) => {
            println!("Touch DOWN");
            // Here we handle touch down events //
            sys_handler::set_bw_mode(3);
            sys_handler::set_default_waveform(3)
            //         let touchstate = 1;
        }
        TouchEvent::Up(_) => {
            println!("Touch UP");
            // Here we handle touch up events //
            //       let touchstate = 0;

            std::thread::sleep(Duration::from_millis(200));

            sys_handler::set_bw_mode(0);
            sys_handler::set_default_waveform(5)
        }
        TouchEvent::Motion(_) => {
            //            let motevt = input::event::touch::TouchEvent::Motion(event);
            //            println!("Touch Motion ({:?})", motevt);
            // Here we handle touch motion events //
            sys_handler::set_bw_mode(2);
            sys_handler::set_default_waveform(1);

            //         let touchstate = 2;
        }
        TouchEvent::Frame(_) => {
            //            println!("{:?}", event);
            // Here we handle touch frame events //
        }
        _ => {
            println!(
                "This touch event didn't match any specific filters, it was: {:?}",
                ev
            );
        }
    }
}
