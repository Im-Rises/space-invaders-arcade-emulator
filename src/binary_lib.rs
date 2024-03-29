pub fn get_bit(data: u8, bit: usize) -> bool {
    ((data >> bit) & 0x1) != 0 //as bool cannot cast ???
}

pub fn set_bit(data: u8, bit: usize) -> u8 {
    data | (1 << bit)
}

pub fn reset_bit(data: u8, bit: usize) -> u8 {
    data & !(1 << bit)
}

pub fn set_reset_bit(data: u8, bit: usize, value: bool) -> u8 {
    if value {
        set_bit(data, bit)
    } else {
        reset_bit(data, bit)
    }
}
