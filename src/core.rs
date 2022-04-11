struct poly_z80
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
    opcode:u8
}

impl poly_z80
{
    fn fetch()
    {
        
    }

    fn imm_addressing()
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
