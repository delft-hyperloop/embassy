pub mod commands;
mod consts;
pub mod control;
pub mod event;
mod helpers;
pub mod indications;
mod macros;
mod opcodes;
pub mod responses;
pub mod runner;
pub mod typedefs;

use core::slice;

use embassy_net_driver_channel as ch;

use crate::bus::Bus;
pub use crate::bus::SpiBusCyw43;
pub use crate::mac::control::{Control, Error as ControlError};
pub use crate::runner::Runner;
pub use crate::structs::BssInfo;
use crate::sub::mac::Mac;

const MTU: usize = 1514;

pub struct State {
    ioctl_state: IoctlState,
    ch: ch::State<MTU, 4, 4>,
    events: Events,
}

impl State {
    pub fn new() -> Self {
        Self {
            ioctl_state: IoctlState::new(),
            ch: ch::State::new(),
            events: Events::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerManagementMode {
    /// Custom, officially unsupported mode. Use at your own risk.
    /// All power-saving features set to their max at only a marginal decrease in power consumption
    /// as oppposed to `Aggressive`.
    SuperSave,

    /// Aggressive power saving mode.
    Aggressive,

    /// The default mode.
    PowerSave,

    /// Performance is prefered over power consumption but still some power is conserved as opposed to
    /// `None`.
    Performance,

    /// Unlike all the other PM modes, this lowers the power consumption at all times at the cost of
    /// a much lower throughput.
    ThroughputThrottling,

    /// No power management is configured. This consumes the most power.
    None,
}

impl Default for PowerManagementMode {
    fn default() -> Self {
        Self::PowerSave
    }
}

impl PowerManagementMode {
    // TODO
}

pub type NetDriver<'a> = ch::Device<'a, MTU>;

pub async fn new<'a, PWR, SPI>(
    state: &'a mut State,
    mac_subsystem: Mac,
    firmware: &[u8],
) -> (NetDriver<'a>, Control<'a>, Runner<'a, PWR, SPI>) {
    let (ch_runner, device) = ch::new(&mut state.ch, [0; 6]);
    let state_ch = ch_runner.state_runner();

    let mut runner = Runner::new(ch_runner, Bus::new(pwr, spi), &state.ioctl_state, &state.events);

    runner.init(firmware).await;

    (
        device,
        Control::new(state_ch, &state.events, &state.ioctl_state),
        runner,
    )
}

fn slice8_mut(x: &mut [u32]) -> &mut [u8] {
    let len = x.len() * 4;
    unsafe { slice::from_raw_parts_mut(x.as_mut_ptr() as _, len) }
}
