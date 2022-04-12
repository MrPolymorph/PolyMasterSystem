struct Poly_Z80<'a>
{
    accumulator:u8,
    b_register:u8,
    c_register:u8,
    d_register:u8,
    e_register:u8,
    f_register:u8,
    h_register:u8,
    l_register:u8,
    /// interrupt page register
    i_register:u8,
    /// memory refresh register
    r_register:u8,
    /// index register x
    ixh_register:u8,
    ixl_register:u8,
    /// index register y
    iyh_register:u8,
    iyl_register:u8,
    /// program counter
    pc:u16,
    /// stack pointer
    sp:u16,
    opcode:u8,
}

impl Poly_Z80
{
    fn new() -> Poly_Z80
    {
        Poly_Z80 {
            accumulator: 0,
            b_register: 0,
            c_register: 0,
            d_register: 0,
            e_register: 0,
            f_register: 0,
            h_register: 0,
            l_register: 0,
            i_register: 0,
            r_register: 0,
            ixh_register: 0,
            ixl_register: 0,
            iyh_register: 0,
            iyl_register: 0,
            pc: 0,
            sp: 0,
            opcode: 0,
        }
    }

    fn fetch()
    {

    }

    fn imm_addressing()
    {

    }

    fn connect_to_bus(bus: Bus)
    {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
