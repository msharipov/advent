pub struct Keypad(u64);

impl Default for Keypad {
    fn default() -> Self {
        Self(5)
    }
}

impl Keypad {
    pub fn up(&mut self) {
        if self.0 > 3 {
            self.0 -= 3;
        }
    }

    pub fn down(&mut self) {
        if self.0 < 7 {
            self.0 += 3;
        }
    }

    pub fn left(&mut self) {
        if self.0 % 3 != 1 {
            self.0 -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.0 % 3 != 0 {
            self.0 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypad_move_test_1() {
        let mut kp = Keypad::default();
        kp.up();
        kp.up();
        kp.left();
        assert_eq!(kp.0, 1);
        kp.left();
        kp.left();
        kp.right();
        kp.right();
        assert_eq!(kp.0, 3);
        kp.down();
        kp.left();
        kp.down();
        assert_eq!(kp.0, 8);
    }
}
