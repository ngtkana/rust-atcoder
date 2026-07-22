pub fn fpu<const P: u64>(value: usize) -> FpBase<P> {
    FpBase::new(value as u64)
}

pub fn fp<const P: u64>(value: u64) -> FpBase<P> {
    FpBase::new(value)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FpBase<const P: u64> {
    value: u64,
}

impl<const P: u64> FpBase<P> {
    pub fn new(value: u64) -> Self {
        Self { value: value % P }
    }
    pub fn pow(mut self: Self, mut exp: u64) -> Self {
        if exp == 0 {
            return fp(1);
        }
        let mut ans = fp(1);
        while exp != 1 {
            if exp & 1 == 1 {
                ans *= self;
            }
            self *= self;
            exp >>= 1;
        }
        ans *= self;
        ans
    }
    pub fn inv(self) -> Self {
        fn euclid(a: u64, m: u64) -> u64 {
            if a == 1 { 1 } else { m + (1 - m * euclid(m % a, a)) / a }
        }
        Self {
            value: euclid(self.value, P),
        }
    }
}

impl<const P: u64> std::fmt::Debug for FpBase<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pub fn berlekamp_massey(a: i64, p: i64) -> [i64; 2] {
            let mut u0 = 0;
            let mut v0 = 1_i64;
            let mut w0 = a * u0 + p * v0;
            let mut u1 = 1;
            let mut v1 = 0;
            let mut w1 = a * u1 + p * v1;
            while p <= w0 * w0 {
                let q = w0 / w1;
                u0 -= q * u1;
                v0 -= q * v1;
                w0 -= q * w1;
                std::mem::swap(&mut u0, &mut u1);
                std::mem::swap(&mut v0, &mut v1);
                std::mem::swap(&mut w0, &mut w1);
            }
            [w0, u0]
        }
        if self.value == 0 {
            return write!(f, "0");
        }
        let [mut num, mut den] = berlekamp_massey(self.value as i64, P as i64);
        if den < 0 {
            num = -num;
            den = -den;
        }
        if den == 1 { write!(f, "{num}") } else { write!(f, "{num}/{den}") }
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

impl<const P: u64> std::ops::MulAssign for FpBase<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const P: u64> std::ops::Neg for FpBase<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.value == 0 {
            self
        } else {
            Self {
                value: P - self.value,
            }
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

// ==========================================
// FFT
// ==========================================
pub fn fft<const P: u64>(items: &mut [FpBase<P>]) {
    assert!(items.len().is_power_of_two());
    let p = (2..P)
        .map(fp::<P>)
        .find(|&p| p.pow((P - 1) / 2) != fp(1))
        .unwrap();
    let mut n = items.len();
    while n != 1 {
        let w = p.pow((P - 1) / n as u64);
        for chunk in items.chunks_mut(n) {
            let (a, b) = chunk.split_at_mut(n / 2);
            let mut coeff = fp(1);
            for (a, b) in a.iter_mut().zip(b) {
                (*a, *b) = (*a + *b, (*a - *b) * coeff);
                coeff *= w;
            }
        }
        n /= 2;
    }
}

pub fn ifft<const P: u64>(items: &mut [FpBase<P>]) {
    assert!(items.len().is_power_of_two());
    let p = (2..P)
        .map(fp::<P>)
        .find(|&p| p.pow((P - 1) / 2) != fp(1))
        .unwrap()
        .inv();
    let mut n = 2;
    while n <= items.len() {
        let w = p.pow((P - 1) / n as u64);
        for chunk in items.chunks_mut(n) {
            let (a, b) = chunk.split_at_mut(n / 2);
            let mut coeff = fp(1);
            for (a, b) in a.iter_mut().zip(b) {
                (*a, *b) = (*a + *b * coeff, *a - *b * coeff);
                coeff *= w;
            }
        }
        n *= 2;
    }
    let len_inv = fp(items.len() as u64).inv();
    for item in items {
        *item *= len_inv;
    }
}
// [End]
// ==========================================
