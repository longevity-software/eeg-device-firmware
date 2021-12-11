//! ADS1299 high level device driver.
//!

pub enum SampleRate {
    SPS_250,
    SPS_500,
    SPS_1000,
    SPS_2000,
    SPS_4000,
    SPS_8000,
    SPS_16000,
}

pub enum Channel {
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
    CH8,
}

pub enum GainSetting {
    X1,
    X2,
    X4,
    X6,
    X8,
    X12,
    X24,
}

pub enum ChannelMode {
    NormalOperation,
    PoweredDownAndShorted,
    Shorted,
    BiasMeasurement,
    SupplyMeasurement,
    TemperatureSensor,
    TestSignal,
    PositiveElectrodeIsBiasDrive,
    NegativeElectrodeIsBiasDrive,
}

pub enum CurrentSourceSetting {
    Off,
    On6NanoAmps,
    On24NanoAmps,
    On6MicroAmps,
    On24MicroAmps,
}

pub struct ADS1299 {}

impl ADS1299 {
    pub fn new() -> ADS1299 {
        return ADS1299 {};
    }

    fn reset() {
        // calls the low level driver to reset the device.
    }

    fn start_data_capture() {
        // calls the low level driver to start data capture
    }

    fn stop_data_capture() {
        // calls the low level driver to stop data capture
    }

    fn set_sample_rate(rate: SampleRate) {
        match rate {
            _ => {}
        }
    }

    fn set_channel_settings(channel: Channel, gain: GainSetting, mode: ChannelMode) {
        // call low
    }

    fn set_current_sources(current: CurrentSourceSetting) {
        // call low
    }
}
