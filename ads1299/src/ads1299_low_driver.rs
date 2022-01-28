use core::marker::PhantomData;

pub struct RegAccessor<R, W> {
    phantom: PhantomData<(R, W)>,
}

impl<R, W> RegAccessor<R, W> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

pub struct Ads1299Device {}

impl Ads1299Device {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_supported_channels(&self) -> u8 {
        let mut temp: RegAccessor<id_reg::R, ()> = RegAccessor::new();

        return temp.read().number_of_channels();
    }
}

pub mod id_reg {
    use super::RegAccessor;

    #[derive(Clone, Copy)]
    pub struct R(u8);

    impl R {
        pub fn number_of_channels(&self) -> u8 {
            match self.0 & 0x03 {
                0x00 => 4,
                0x01 => 6,
                0x02 => 8,
                _ => 0,
            }
        }
    }

    impl RegAccessor<R, ()> {
        pub fn read(&mut self) -> R {
            R(0x01)
        }
    }
}
