use super::*;

const MAX_EIRP: u8 = 14;
const JOIN_CHANNELS: [u32; 3] = [922_100_000, 922_300_000, 922_500_000];
const DEFAULT_RX2: u32 = 921_900_000;

pub(crate) type KR920 = DynamicChannelPlan<KR920Region>;

#[derive(Default, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct KR920Region;

const _: () = {
    impl<R: DynamicChannelRegion> DynamicChannelPlan<R> {
        pub fn new_kr920() -> Self {
            Self::new(as920_freq_check)
        }
    }
    fn as920_freq_check(f: u32) -> bool {
        (920_900_000..=923_300_000).contains(&f)
    }
};

impl ChannelRegion for KR920Region {
    fn datarates() -> &'static [Option<Datarate>; NUM_DATARATES as usize] {
        &DATARATES
    }
    // this should be a function of both pw & frequency (10dBm for _ <= 921.9, 14dBm for 922.1 <= _ )
    fn tx_power_adjust(pw: u8) -> Option<u8> {
        match pw {
            0..=7 => Some(MAX_EIRP - (2 * pw)),
            _ => None,
        }
    }
}

impl DynamicChannelRegion for KR920Region {
    fn join_channels() -> u8 {
        JOIN_CHANNELS.len() as u8
    }

    fn get_default_rx2() -> u32 {
        DEFAULT_RX2
    }

    fn init_channels(channels: &mut ChannelPlan) {
        for (i, freq) in JOIN_CHANNELS.iter().enumerate() {
            channels[i] = Some(Channel::new(*freq, DR::_0, DR::_5));
        }
    }
}

use super::{Bandwidth, Datarate, SpreadingFactor};

pub(crate) const DATARATES: [Option<Datarate>; NUM_DATARATES as usize] = [
    Some(Datarate {
        spreading_factor: SpreadingFactor::_12,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 59,
    }),
    Some(Datarate {
        spreading_factor: SpreadingFactor::_11,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 59,
    }),
    Some(Datarate {
        spreading_factor: SpreadingFactor::_10,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 59,
        max_mac_payload_size_with_dwell_time: 59,
    }),
    Some(Datarate {
        spreading_factor: SpreadingFactor::_9,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 123,
        max_mac_payload_size_with_dwell_time: 123,
    }),
    Some(Datarate {
        spreading_factor: SpreadingFactor::_8,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    Some(Datarate {
        spreading_factor: SpreadingFactor::_7,
        bandwidth: Bandwidth::_125KHz,
        max_mac_payload_size: 250,
        max_mac_payload_size_with_dwell_time: 250,
    }),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
