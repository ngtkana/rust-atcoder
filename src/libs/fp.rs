pub const fn fpu<const P: u64>(value: usize) -> Fp<P> {
    Fp::new(value as u64)
}

pub const fn fp<const P: u64>(value: u64) -> Fp<P> {
    Fp::new(value)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fp<const P: u64> {
    value: u64,
}

impl<const P: u64> Fp<P> {
    pub const fn new(value: u64) -> Self {
        Self { value: value % P }
    }
    pub const fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value % P,
        }
    }
    pub const fn pow(mut self: Self, mut exp: u64) -> Self {
        if exp == 0 {
            return fp(1);
        }
        let mut ans = fp(1);
        while exp != 1 {
            if exp & 1 == 1 {
                ans = ans.mul(self);
            }
            self = self.mul(self);
            exp >>= 1;
        }
        ans.mul(self)
    }
    pub const fn inv(self) -> Self {
        const fn euclid(a: i64, m: i64) -> i64 {
            if a == 1 { 1 } else { m + (1 - m * euclid(m % a, a)) / a }
        }
        Self {
            value: euclid(self.value as i64, P as i64) as u64,
        }
    }
}

impl<const P: u64> std::fmt::Debug for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pub const fn berlekamp_massey(a: i64, p: i64) -> [i64; 2] {
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

impl<const P: u64> std::fmt::Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ==========================================
// Arithmetic
// ==========================================
impl<const P: u64> std::ops::Add for Fp<P> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<const P: u64> std::ops::AddAssign for Fp<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        if P <= self.value {
            self.value -= P;
        }
    }
}
impl<const P: u64> std::ops::Sub for Fp<P> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<const P: u64> std::ops::SubAssign for Fp<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.value < rhs.value {
            self.value += P;
        }
        self.value -= rhs.value;
    }
}
impl<const P: u64> std::ops::Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}
impl<const P: u64> std::ops::MulAssign for Fp<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const P: u64> std::ops::Div for Fp<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
impl<const P: u64> std::ops::DivAssign for Fp<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self) / rhs
    }
}

impl<const P: u64> std::ops::Neg for Fp<P> {
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

// ==========================================
// Iterators
// ==========================================
impl<const P: u64> std::iter::Sum for Fp<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fp::new(0), |acc, item| acc + item)
    }
}

impl<'a, const P: u64> std::iter::Sum<&'a Self> for Fp<P> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Fp::new(0), |acc, &item| acc + item)
    }
}

// ==========================================
// FFT
// ==========================================
const TWIDDLES_LEN: usize = 64;
pub const fn find_primitive_root<const P: u64>() -> Fp<P> {
    let mut x = fp(2);
    while x.value != P {
        if x.pow((P - 1) / 2).value == 1 {
            return x;
        }
        x.value += 1;
    }
    panic!("primitive root not found");
}
const fn build_twiddles<const P: u64>(root: Fp<P>) -> [Fp<P>; TWIDDLES_LEN] {
    let mut result = [fp(0); TWIDDLES_LEN];
    let k = (P - 1).trailing_zeros();
    let mut i = k as usize - 1;
    result[i] = root.pow((P - 1) >> k);
    while i != 0 {
        result[i - 1] = fp(result[i].value * result[i].value % P);
        i -= 1;
    }
    result
}

pub fn fft<const P: u64>(items: &mut [Fp<P>]) {
    assert!(items.len().is_power_of_two());
    assert!(items.len().trailing_zeros() <= (P - 1).trailing_zeros());
    let w = const { build_twiddles(find_primitive_root()) };
    let mut n = items.len();
    while n != 1 {
        let w = w[n.trailing_zeros() as usize];
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

pub fn ifft<const P: u64>(items: &mut [Fp<P>]) {
    assert!(items.len().is_power_of_two());
    assert!(items.len().trailing_zeros() <= (P - 1).trailing_zeros());
    let w = const { build_twiddles(find_primitive_root().inv()) };
    let mut n = 2;
    while n <= items.len() {
        let w = w[n.trailing_zeros() as usize];
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
// ==========================================

// ==========================================
// Newton
// ==========================================
pub fn fps_inv<const P: u64>(f: &[Fp<P>], precision: usize) -> Vec<Fp<P>> {
    let fft_len_max = precision.next_power_of_two();
    let mut g = vec![fp(0); precision];
    g[0] = f[0].inv();
    let mut h = vec![fp(0); fft_len_max];
    let mut g_fft = vec![fp(0); fft_len_max];
    let mut fft_len = 2;
    while fft_len <= fft_len_max {
        if fft_len < f.len() {
            h[..fft_len].copy_from_slice(&f[..fft_len]);
        } else {
            h[..f.len()].copy_from_slice(&f[..f.len()]);
            h[f.len()..fft_len].fill(fp(0));
        }
        for i in 0..fft_len / 2 {
            g_fft[i] = g[i];
        }
        fft(&mut h[..fft_len]);
        fft(&mut g_fft[..fft_len]);
        for i in 0..fft_len {
            h[i] = fp(1) - h[i] * g_fft[i];
        }
        ifft(&mut h[..fft_len]);
        h[..fft_len / 2].fill(fp(0));
        fft(&mut h[..fft_len]);
        for i in 0..fft_len {
            h[i] = h[i] * g_fft[i];
        }
        ifft(&mut h[..fft_len]);
        g[fft_len / 2..fft_len.min(precision)]
            .copy_from_slice(&h[fft_len / 2..fft_len.min(precision)]);
        fft_len *= 2;
    }
    g
}

pub fn poly_mul<const P: u64>(mut a: Vec<Fp<P>>, mut b: Vec<Fp<P>>) -> Vec<Fp<P>> {
    let result_len = a.len() + b.len() - 1;
    let fft_len = result_len.next_power_of_two() * 2;
    a.resize(fft_len, fp(0));
    b.resize(fft_len, fp(0));
    fft(&mut a);
    fft(&mut b);
    for i in 0..fft_len {
        a[i] = a[i] * b[i];
    }
    ifft(&mut a);
    a.truncate(result_len);
    a
}

pub fn poly_div_rem<const P: u64>(
    mut a: Vec<Fp<P>>,
    mut b: Vec<Fp<P>>,
) -> (Vec<Fp<P>>, Vec<Fp<P>>) {
    assert_ne!(*b.last().unwrap(), fp(0));
    if a.len() < b.len() {
        return (vec![], a);
    }
    let d = b.iter().position(|&b| b != fp(0)).unwrap();
    a[d..].reverse();
    b[d..].reverse();
    let precision = a.len() - b.len() + 1;
    let mut q = poly_mul(a[d..].to_vec(), fps_inv(&b[d..], precision));
    q.truncate(precision);
    q.reverse();
    a[d..].reverse();
    b[d..].reverse();
    let bq = poly_mul(b, q.to_vec());
    for i in 0..bq.len() {
        a[i] -= bq[i];
    }
    while a.pop_if(|&mut a| a == fp(0)).is_some() {}
    (q, a)
}

pub fn multipoint_evaluation<const P: u64>(
    f: Vec<Fp<P>>,
    points: impl ExactSizeIterator<Item = Fp<P>>,
) -> Vec<Fp<P>> {
    let n = points.len();
    let mut prod = vec![vec![]; n * 2];
    for (prod, point) in prod[n..].iter_mut().zip(points) {
        *prod = vec![-point, fp(1)];
    }
    for i in (1..n).rev() {
        prod[i] = poly_mul(prod[2 * i].clone(), prod[2 * i + 1].clone());
    }
    let mut rem = vec![vec![]; n * 2];
    rem[1] = poly_div_rem(f, prod[1].clone()).1;
    for i in 1..n {
        rem[2 * i] = poly_div_rem(rem[i].clone(), prod[2 * i].clone()).1;
        rem[2 * i + 1] = poly_div_rem(rem[i].clone(), prod[2 * i + 1].clone()).1;
    }
    rem[n..].into_iter().map(|ans| ans[0]).collect()
}
