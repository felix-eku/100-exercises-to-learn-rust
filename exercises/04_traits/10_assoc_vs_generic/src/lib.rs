trait Power<E = Self> {
    fn power(self, n: E) -> Self;
}

impl Power for u32 {
    fn power(self, n: Self) -> Self {
        let mut power = 1;
        for _ in 1..=n {
            power *= self;
        }
        power
    }
}

impl Power<u16> for u32 {
    fn power(self, n: u16) -> Self {
        let mut power = 1;
        for _ in 1..=n {
            power *= self;
        }
        power
    }
}

impl Power<&u32> for u32 {
    fn power(self, n: &u32) -> Self {
        let mut power = 1;
        for _ in 1..=*n {
            power *= self;
        }
        power
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
