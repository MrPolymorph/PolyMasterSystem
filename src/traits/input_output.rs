trait input_output
{
    fn connect(io: input_output);
    fn read(address: u16) -> u8;
    fn write(address: u16, data: u8);
}