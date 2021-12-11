//! Center for all EEG data to be funnelled through.
//!

pub struct EegData {
    channel_data: [i32; 32],
    impedance_data: [u16; 32],
}

pub struct DataCenter {
    // public

    // private
    data_buffer: [EegData; 1000],
}
impl Copy for EegData {}

impl Clone for EegData {
    fn clone(&self) -> Self {
        Self {
            channel_data: self.channel_data.clone(),
            impedance_data: self.impedance_data.clone(),
        }
    }
}

impl DataCenter {
    pub fn new() -> DataCenter {
        DataCenter {
            data_buffer: [EegData {
                channel_data: [0i32; 32],
                impedance_data: [0u16; 32],
            }; 1000],
        }
    }
}
