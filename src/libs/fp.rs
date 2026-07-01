pub fn new<const P: u64>(value: u64) -> FpBase<P> {
    FpBase::new(value)
}

#[derive(Debug, Clone, Copy)]
pub struct FpBase<const P: u64> {
    value: u64,
}

impl<const P: u64> FpBase<P> {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

impl<const P: u64> std::fmt::Display for FpBase<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ==========================================
// Arithmetic
// ==========================================
impl<const P: u64> std::ops::Add for FpBase<P> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const P: u64> std::ops::AddAssign for FpBase<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if P <= self.value {
            self.value -= P;
        }
    }
}

impl<const P: u64> std::ops::Sub for FpBase<P> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const P: u64> std::ops::SubAssign for FpBase<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += P;
        }
        self.value -= rhs.value;
    }
}

impl<const P: u64> std::ops::Mul for FpBase<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value % P,
        }
    }
}
// [End]
// ==========================================

impl<const P: u64> std::iter::Sum for FpBase<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FpBase::new(0), |acc, item| acc + item)
    }
}

impl<'a, const P: u64> std::iter::Sum<&'a Self> for FpBase<P> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(FpBase::new(0), |acc, &item| acc + item)
    }
}
