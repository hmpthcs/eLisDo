# eLisDo

## What
Listens for user-configured input events, then triggers user-configured actions. Maybe will listen for window-manager events in the future, too. Seeking quickest and most reliable means for doing so under wayland, possibly framebuffer later on. Overarching goals include finding creative ways to to smoothly transition from HQ to fast waveforms and back.

WARNING: work in progress; probably doesn't funciton as intended. Issues, pull requests and commentary welcome. 

## Why
### One use case: Pinenote display driver parameters
With an e-ink device, you get fast black-and-white-only screen refreshes or slow full 16-level grayscale refreshes (or somewhere in between with 4-levels of grayscale).
Having all 16 shades of gray makes a huge difference and is a selling point of modern e-ink panels. High-res photos look sharp, UI items are easier to distinguish, etc. It’s far too slow, however, for any meaningful human interaction beyond high-latency page-flipping. In order to interact with any on-screen items with a finger, stylus or keyboard, using a faster refresh mode is an absolute must to achieve acceptable latency.
Modes can be set in userspace via kernel module parameters. Currently, extent of convenience in panel control stops at changing modes with the press of a button somewhere in the user interface, or perhaps even running a shell script which switches modes when called. Users might even set some timers to have modes return to defaults after a little while. This works to some extent, but is far from a seamless approach.
Here, we allow the user to set up automatic triggers that change modes when needed, then return immediately to a user-defined default. A daemon runs in userspace that allows synchronizing e-ink driver parameter states with device states.

### Further pursuits
Ultimately, a desktop environment / compositor / window manager ought to integrate these functionalities. After learning through experiments with a detached daemon, the next steps might involve running bits of this code within a wayland compositor. This would allow better thread prioritization,  predictability and control over timing with respect to frame redraws and damage tracking. In addition, an API can be exposed in the form of a wayland protocol extension, allowing compositors to optimize display interactions for eink-aware applications.

## How

### CORE CONCEPT
#### Simple events first
Bare-bones passthrough. One triggering event:one action. Optimize and test speed here.


[ hardware | filesystem event ]		→	exec [ shell script | binary ]

#### Complex events next
Adds internal logic, enabling  things like touch gestures, keyboard chords, and more:


1st  [ hardware | filesystem event ]	→	[ change internal state ] (e.g. await another event)
2nd  [ hardware | filesystem event ]	→	exec [ shell script | binary ]


### IMPLEMENTATION
Implementation I: libinput

#### Written in rust
Input listener: all lifted from smithay/input-rs
EPD actions: all lifted from m-weigand/pinenote-dbus-service

#### Implementation II: evdev 
Just an idea…
Can’t figure out evdev-rs stuff… halp


## Status 
Working rudimentary / POC Libinput implementation. No Evdev version yet…

### Events available:
- [X] Stylus proximity in/out

- [X] Touch down/up/motion/frame

### Actions available:
- [X] Set rockchip-ebc module parameters

### Todo:
- [ ] Config file: 
  - [ ] specify input events to listen, actions in response to each
- [ ] Organize source file structure:
  - [ ] main.rs (main loop ; read config here ? )
  - [ ] listeners.rs (listens for events, passes them over to handlers)
  - [ ] handlers.rs (receives events from the listeners; tracks internal state, triggers effectors when appropriate) 
  - [ ] effectors.rs (does stuff when the handler tells it to)
