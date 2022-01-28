//! ADS1299 high level device driver.
//!

use crate::ads1299_low_driver;

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

#[derive(Copy, Clone)]
pub enum GainSetting {
    X1,
    X2,
    X4,
    X6,
    X8,
    X12,
    X24,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum CurrentSourceSetting {
    Off,
    On6NanoAmps,
    On24NanoAmps,
    On6MicroAmps,
    On24MicroAmps,
}

#[derive(Copy, Clone)]
pub struct ADS1299_CHANNEL {
    gainSetting: GainSetting,
    mode: ChannelMode,
}

pub struct ADS1299 {
    sampleRate: SampleRate,
    maxChannels: u8,
    channels: [ADS1299_CHANNEL; 8],
    currentSourceSetting: CurrentSourceSetting,
}

impl ADS1299 {
    pub fn new() -> ADS1299 {
        let default_channel = ADS1299_CHANNEL {
            gainSetting: crate::ads1299_driver::GainSetting::X1,
            mode: crate::ads1299_driver::ChannelMode::PoweredDownAndShorted,
        };

        let device = ads1299_low_driver::Ads1299Device::new();

        return ADS1299 {
            sampleRate: crate::ads1299_driver::SampleRate::SPS_250,
            maxChannels: device.get_supported_channels(),
            channels: [
                default_channel,
                default_channel,
                default_channel,
                default_channel,
                default_channel,
                default_channel,
                default_channel,
                default_channel,
            ],
            currentSourceSetting: crate::ads1299_driver::CurrentSourceSetting::Off,
        };
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

    fn set_sample_rate(&mut self, rate: SampleRate) {
        self.sampleRate = rate;

        // Call low level code
    }

    fn set_channel_settings(&mut self, channel: Channel, gain: GainSetting, mode: ChannelMode) {
        let index = match channel {
            Channel::CH1 => 0,
            Channel::CH2 => 1,
            Channel::CH3 => 2,
            Channel::CH4 => 3,
            Channel::CH5 => 4,
            Channel::CH6 => 5,
            Channel::CH7 => 6,
            Channel::CH8 => 7,
        };

        // update our local copies
        self.channels[index].gainSetting = gain;
        self.channels[index].mode = mode;

        // call low
    }

    fn set_current_sources(&mut self, current: CurrentSourceSetting) {
        self.currentSourceSetting = current;

        // call low
    }
}
