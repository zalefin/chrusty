
pub struct Packet {
    pub opcode: u8,
    pub src: u64,
    pub dst: u64,
    pub body: String,
}

impl Packet {
    pub fn print(&self) {
        println!("Op({}), ({}, {}) -> \"{}\"", self.opcode, self.src, self.dst, self.body);
    }
}

