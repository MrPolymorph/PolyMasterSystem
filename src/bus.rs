use std::ptr;
use std::ptr::null;
use crate::Poly_Z80;


pub struct Bus
{
    cpu: Poly_Z80,
}

impl Bus {
    fn new(poly_cpu: Poly_Z80) -> Bus {
        return Bus { cpu: poly_cpu };
    }

    fn write(address: u16, data: u8)
    {}

    fn read(address: u16) -> u8
    {
        return 1;
    }

    fn reset()
    {}

    fn clock()
    {}
}
