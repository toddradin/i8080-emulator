pub trait MemoryMap {
    fn load_rom(&mut self);

    fn read(&mut self, addr: u16) -> u8;

    fn read_slice(&mut self, addr: u16) -> &[u8];

    fn write(&mut self, addr: u16, val: u8);
}
