#[derive(Debug)]
pub struct File {
    data: Vec<u8>,
    pointer: isize,
}

impl File {
    pub fn new(data: Vec<u8>) -> File {
        File { data, pointer: 0}
    }
}

impl File {
    pub fn offset_pointer(&mut self, offset: isize) {
        self.pointer += offset;
    }

    pub fn get_pointer(&mut self) -> usize {
        self.pointer as usize
    }

    pub fn get_u1(&mut self) -> u8 {
        let out = self.data[self.pointer as usize];
        self.pointer += 1;
        out
    }

    pub fn get_u2(&mut self) -> u16 {
        let out = ((self.data[self.pointer as usize] as u16) << 8) + ((self.data[(self.pointer + 1)  as usize] as u16));
        self.pointer += 2;
        out
    }

    pub fn get_u4(&mut self) -> u32 {
        let out = ((self.data[self.pointer  as usize] as u32) << 24) + ((self.data[(self.pointer + 1)  as usize] as u32) << 16) + ((self.data[(self.pointer + 2)  as usize] as u32) << 8) + ((self.data[(self.pointer + 3) as usize] as u32));
        self.pointer += 4;
        out
    }

    pub fn get_range(&mut self, range: isize) -> Vec<u8> {
        let out = self.data[(self.pointer) as usize..(self.pointer+range) as usize].to_vec();
        self.pointer += range;
        out
    }

    pub fn get_range_u2(&mut self, range: usize) -> Vec<u16> {
        (0..range).into_iter().map(|f| {self.get_u2()}).collect()
    }

    pub fn has_next(&mut self) -> bool {
        (self.pointer as usize) < self.data.len()
    }
}