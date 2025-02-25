use super::*;

const JOIN_CHANNELS: [u32; 3] = [922_100_000, 922_300_000, 922_500_000];
const DEFAULT_RX2: u32 = 921_900_000;

pub(crate) type KR920 = DynamicChannelPlan<3, 6, KR920Region>;

#[derive(Default, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct KR920Region;

impl ChannelRegion<6> for KR920Region {
    fn datarates() -> &'static [Option<Datarate>; 6] {
        &DATARATES
    }
}

impl DynamicChannelRegion<3, 6> for KR920Region {
    fn join_channels() -> [u32; 3] {
        JOIN_CHANNELS
    }

    fn get_default_rx2() -> u32 {
        DEFAULT_RX2
    }
}

use super::{Bandwidth, Datarate, SpreadingFactor};

pub(crate) const DATARATES: [Option<Datarate>; 6] = [
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
];
