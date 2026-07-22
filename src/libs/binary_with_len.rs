#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct BinaryWithLen(usize);
impl BinaryWithLen {
    pub fn new(value: usize, len: u32) -> Self {
        Self(value | 1 << len)
    }
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
    pub fn poped(self) -> Self {
        Self(self.0 >> 1)
    }
    pub fn pushed(self, bit: usize) -> Self {
        Self(self.0 << 1 | bit)
    }
    pub fn prefixes(self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self), |&x| Some(x.poped())).take_while(Self::is_valid)
    }
    pub fn len(&self) -> u32 {
        self.0.ilog2()
    }
    pub fn into_raw(self) -> usize {
        self.0
    }
    pub fn into_value(self, expected_len: u32) -> usize {
        let result = self.0 ^ 1 << expected_len;
        assert!(result < 1 << expected_len);
        result
    }
}

impl std::fmt::Debug for BinaryWithLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "MsbMasked(0)"),
            1 => write!(f, "\"\""),
            x => {
                let p = x.ilog2();
                write!(f, "\"{:0width$b}\"", x ^ (1 << p), width = p as usize)
            }
        }
    }
}
