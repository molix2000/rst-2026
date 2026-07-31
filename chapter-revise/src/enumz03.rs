// Source - https://stackoverflow.com/q/51777187
// Posted by linuxuser, modified by community. See post 'Timeline' for change history
// Retrieved 2026-07-27, License - CC BY-SA 4.0

pub struct Something {
    value: usize,
}

impl Something {
    pub fn get_and_increment(&mut self) -> u8 {
        let res = self.get();
        self.value += 1;

        res
    }

    pub fn get(&self) -> u8 {
        3
    }
}

