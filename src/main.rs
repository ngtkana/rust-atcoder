use crate::fp_fft::poly_mul;
use fp::fp;
use proconio::input;
use std::collections::VecDeque;

const P: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut queue = a
        .iter()
        .copied()
        .map(|a| vec![fp(1), fp::<P>(a)])
        .collect::<VecDeque<_>>();
    while queue.len() != 1 {
        let a = queue.pop_front().unwrap();
        let b = queue.pop_front().unwrap();
        queue.push_back(poly_mul(&a, &b));
    }
    let f = queue.pop_front().unwrap();

    let mut inv = vec![fp::<P>(1); 1 + n];
    for i in 2..=n {
        let q = P / i as u64;
        let r = (P - q * i as u64) as usize;
        inv[i] = inv[r] * fp(P - q);
    }
    let mut coeff = fp::<P>(1);
    let mut ans = fp::<P>(0);
    for e in 0..=n.min(k as usize) {
        if e != 0 {
            coeff *= fp(k - e as u64 + 1);
            coeff *= inv[n];
        }
        ans += coeff * f[n - e];
    }
    println!("{ans}");
}

// fp {{{
// https://ngtkana.github.io/ac-adapter-rs/fp/index.html
#[allow(unused_imports)]
#[allow(dead_code)]
mod fp {
    use std::iter::Product;
    use std::iter::Sum;
    use std::mem::swap;
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Div;
    use std::ops::DivAssign;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Neg;
    use std::ops::Sub;
    use std::ops::SubAssign;
    #[macro_export]
    macro_rules! fp {
        ($value:expr) => {
            $crate::fp::Fp::from($value)
        };
        ($value:expr; mod $p:expr) => {
            $crate::fp::Fp::<$p>::from($value)
        };
    }
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Fp<const P: u64> {
        value: u64,
    }
    pub fn fp<const P: u64>(value: u64) -> Fp<P> {
        Fp::new(value)
    }
    impl<const P: u64> Fp<P> {
        pub const fn new(value: u64) -> Self {
            Self { value: value % P }
        }
        pub const fn value(self) -> u64 {
            self.value
        }
        pub fn inv(self) -> Self {
            Self {
                value: mod_inv::<P>(self.value),
            }
        }
        pub fn pow(self, mut exp: u64) -> Self {
            let mut result = Self::new(1);
            let mut base = self;
            while exp > 0 {
                if exp & 1 == 1 {
                    result *= base;
                }
                base *= base;
                exp >>= 1;
            }
            result
        }
        pub fn sign(exp: usize) -> Self {
            Self::new(if exp % 2 == 0 { 1 } else { P - 1 })
        }
    }
    impl<const P: u64> std::fmt::Debug for Fp<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            pub fn berlekamp_massey_fp(a: i64, p: i64) -> [i64; 2] {
                let mut u0 = 0_i64;
                let mut v0 = 1_i64;
                let mut w0 = a * u0 + p * v0;
                let mut u1 = 1_i64;
                let mut v1 = 0_i64;
                let mut w1 = a * u1 + p * v1;
                while p <= w0 * w0 {
                    let q = w0 / w1;
                    u0 -= q * u1;
                    v0 -= q * v1;
                    w0 -= q * w1;
                    swap(&mut u0, &mut u1);
                    swap(&mut v0, &mut v1);
                    swap(&mut w0, &mut w1);
                }
                [w0, u0]
            }
            if self.value == 0 {
                return write!(f, "0");
            }
            let [mut num, mut den] = berlekamp_massey_fp(self.value as i64, P as i64);
            if den < 0 {
                num = -num;
                den = -den;
            }
            if den == 1 { write!(f, "{num}") } else { write!(f, "{num}/{den}") }
        }
    }
    impl<const P: u64> std::fmt::Display for Fp<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value())
        }
    }
    macro_rules! impl_from_signed {
        ($($t:ty),*) => {
            $(
                impl<const P: u64> From<$t> for Fp<P> {
                    fn from(x: $t) -> Self {
                        if x < 0 {
                            -Self::new((P as i64 - x as i64) as u64)
                        } else {
                            Self::new(x as u64)
                        }
                    }
                }
            )*
        };
    }
    impl_from_signed!(i8, i16, i32, i64, i128, isize);
    macro_rules! impl_from_unsigned {
        ($($t:ty),*) => {
            $(
                impl<const P: u64> From<$t> for Fp<P> {
                    fn from(x: $t) -> Self { Self::new(x as u64) }
                }
            )*
        };
    }
    impl_from_unsigned!(u8, u16, u32, u64, u128, usize);
    impl<const P: u64> AddAssign<Fp<P>> for Fp<P> {
        fn add_assign(&mut self, rhs: Fp<P>) {
            self.value += rhs.value;
            if self.value >= P {
                self.value -= P;
            }
        }
    }
    impl<const P: u64> SubAssign<Fp<P>> for Fp<P> {
        fn sub_assign(&mut self, rhs: Fp<P>) {
            if self.value < rhs.value {
                self.value += P;
            }
            self.value -= rhs.value;
        }
    }
    impl<const P: u64> MulAssign<Fp<P>> for Fp<P> {
        fn mul_assign(&mut self, rhs: Fp<P>) {
            self.value = self.value * rhs.value % P;
        }
    }
    #[allow(clippy::suspicious_op_assign_impl)]
    impl<const P: u64> DivAssign<Fp<P>> for Fp<P> {
        fn div_assign(&mut self, rhs: Fp<P>) {
            *self *= rhs.inv();
        }
    }
    macro_rules! fp_forward_ops {
        ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
        )*) => {$(
            impl<const P: u64> $trait_assign<&Fp<P>> for Fp<P> {
                fn $fn_assign(&mut self, rhs: &Fp<P>) {
                    self.$fn_assign(*rhs);
                }
            }
            impl<const P: u64, T: Into<Fp<P>>> $trait<T> for Fp<P> {
                type Output = Fp<P>;
                fn $fn(mut self, rhs: T) -> Self::Output {
                    self.$fn_assign(rhs.into());
                    self
                }
            }
            impl<const P: u64> $trait<&Fp<P>> for Fp<P> {
                type Output = Fp<P>;
                fn $fn(self, rhs: &Fp<P>) -> Self::Output {
                    self.$fn(*rhs)
                }
            }
            impl<const P: u64, T: Into<Fp<P>>> $trait<T> for &Fp<P> {
                type Output = Fp<P>;
                fn $fn(self, rhs: T) -> Self::Output {
                    (*self).$fn(rhs.into())
                }
            }
            impl<const P: u64> $trait<&Fp<P>> for &Fp<P> {
                type Output = Fp<P>;
                fn $fn(self, rhs: &Fp<P>) -> Self::Output {
                    (*self).$fn(*rhs)
                }
            }
        )*};
    }
    fp_forward_ops! {
        Add, AddAssign, add, add_assign,
        Sub, SubAssign, sub, sub_assign,
        Mul, MulAssign, mul, mul_assign,
        Div, DivAssign, div, div_assign,
    }
    impl<const P: u64> Neg for Fp<P> {
        type Output = Fp<P>;
        fn neg(mut self) -> Self::Output {
            if self.value > 0 {
                self.value = P - self.value;
            }
            self
        }
    }
    impl<const P: u64> Sum for Fp<P> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }
    impl<'a, const P: u64> Sum<&'a Self> for Fp<P> {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.copied().sum()
        }
    }
    impl<const P: u64> Product for Fp<P> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }
    impl<'a, const P: u64> Product<&'a Self> for Fp<P> {
        fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.copied().product()
        }
    }
    pub fn mod_inv<const P: u64>(x: u64) -> u64 {
        debug_assert!(P % 2 == 1);
        debug_assert!(P < 1 << 31);
        debug_assert!(x < P);
        mod_inv_signed(x as i64, P as i64) as u64
    }
    fn mod_inv_signed(a: i64, m: i64) -> i64 {
        debug_assert!(a > 0);
        debug_assert!(m > 0);
        if a == 1 {
            return 1;
        }
        m + (1 - m * mod_inv_signed(m % a, a)) / a
    }
}
// }}}
// fp_fft {{{
// https://ngtkana.github.io/ac-adapter-rs/fp_fft/index.html
#[allow(unused_imports)]
#[allow(dead_code)]
mod fp_fft {
    use crate::fp::Fp;
    use crate::fp::mod_inv;
    const P1: u64 = 924_844_033;
    const P2: u64 = 998_244_353;
    const P3: u64 = 1_012_924_417;
    type F1 = Fp<P1>;
    type F2 = Fp<P2>;
    type F3 = Fp<P3>;
    pub trait PrimitiveRoot<const P: u64> {
        const VALUE: Fp<P>;
    }
    impl PrimitiveRoot<P1> for () {
        const VALUE: Fp<P1> = Fp::new(5);
    }
    impl PrimitiveRoot<P2> for () {
        const VALUE: Fp<P2> = Fp::new(3);
    }
    impl PrimitiveRoot<P3> for () {
        const VALUE: Fp<P3> = Fp::new(5);
    }
    pub fn poly_mul<const P: u64>(a: impl AsRef<[Fp<P>]>, b: impl AsRef<[Fp<P>]>) -> Vec<Fp<P>>
    where
        (): PrimitiveRoot<P>,
    {
        let a = a.as_ref();
        let b = b.as_ref();
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        let n = a.len() + b.len() - 1;
        let len = n.next_power_of_two();
        a.resize(len, Fp::new(0));
        b.resize(len, Fp::new(0));
        fft(&mut a);
        fft(&mut b);
        for (a, b) in a.iter_mut().zip(b.iter()) {
            *a *= *b;
        }
        ifft(&mut a);
        a.truncate(n);
        a
    }
    pub fn any_mod_fps_mul<const P: u64>(a: &[Fp<P>], b: &[Fp<P>]) -> Vec<Fp<P>> {
        let v1 = poly_mul(
            a.iter().map(|&x| F1::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| F1::new(x.value())).collect::<Vec<_>>(),
        );
        let v2 = poly_mul(
            a.iter().map(|&x| F2::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| F2::new(x.value())).collect::<Vec<_>>(),
        );
        let v3 = poly_mul(
            a.iter().map(|&x| F3::new(x.value())).collect::<Vec<_>>(),
            b.iter().map(|&x| F3::new(x.value())).collect::<Vec<_>>(),
        );
        v1.into_iter()
            .zip(v2)
            .zip(v3)
            .map(|((e1, e2), e3)| garner(e1, e2, e3))
            .collect::<Vec<_>>()
    }
    pub fn fft<const P: u64>(f: &mut [Fp<P>])
    where
        (): PrimitiveRoot<P>,
    {
        let n = f.len();
        assert!(n.is_power_of_two());
        assert!((P - 1) % n as u64 == 0);
        let mut root = <() as PrimitiveRoot<P>>::VALUE.pow((P - 1) / f.len() as u64);
        let fourth = <() as PrimitiveRoot<P>>::VALUE.pow((P - 1) / 4);
        let mut fft_len = n;
        while 4 <= fft_len {
            let quarter = fft_len / 4;
            for f in f.chunks_mut(fft_len) {
                let mut c = Fp::new(1);
                for (((i, j), k), l) in (0..)
                    .zip(quarter..)
                    .zip(quarter * 2..)
                    .zip(quarter * 3..)
                    .take(quarter)
                {
                    let c2 = c * c;
                    let x = f[i] + f[k];
                    let y = f[j] + f[l];
                    let z = f[i] - f[k];
                    let w = fourth * (f[j] - f[l]);
                    f[i] = x + y;
                    f[j] = c2 * (x - y);
                    f[k] = c * (z + w);
                    f[l] = c2 * c * (z - w);
                    c *= root;
                }
            }
            root *= root;
            root *= root;
            fft_len = quarter;
        }
        if fft_len == 2 {
            for f in f.chunks_mut(2) {
                let x = f[0];
                let y = f[1];
                f[0] = x + y;
                f[1] = x - y;
            }
        }
    }
    pub fn ifft<const P: u64>(f: &mut [Fp<P>])
    where
        (): PrimitiveRoot<P>,
    {
        let n = f.len();
        assert!(n.is_power_of_two());
        let root = <() as PrimitiveRoot<P>>::VALUE.pow((P - 1) / f.len() as u64);
        let mut roots = std::iter::successors(Some(root.inv()), |x| Some(x * x))
            .take(n.trailing_zeros() as usize + 1)
            .collect::<Vec<_>>();
        roots.reverse();
        let fourth = <() as PrimitiveRoot<P>>::VALUE.pow((P - 1) / 4).inv();
        let mut quarter = 1_usize;
        if n.trailing_zeros() % 2 == 1 {
            for f in f.chunks_mut(2) {
                let x = f[0];
                let y = f[1];
                f[0] = x + y;
                f[1] = x - y;
            }
            quarter = 2;
        }
        while quarter != n {
            let fft_len = quarter * 4;
            let root = roots[fft_len.trailing_zeros() as usize];
            for f in f.chunks_mut(fft_len) {
                let mut c = Fp::new(1);
                for (((i, j), k), l) in (0..)
                    .zip(quarter..)
                    .zip(quarter * 2..)
                    .zip(quarter * 3..)
                    .take(quarter)
                {
                    let c2 = c * c;
                    let x = f[i] + c2 * f[j];
                    let y = f[i] - c2 * f[j];
                    let z = c * (f[k] + c2 * f[l]);
                    let w = fourth * c * (f[k] - c2 * f[l]);
                    f[i] = x + z;
                    f[j] = y + w;
                    f[k] = x - z;
                    f[l] = y - w;
                    c *= root;
                }
            }
            quarter = fft_len;
        }
        let d = Fp::from(f.len()).inv();
        for x in f.iter_mut() {
            *x *= d;
        }
    }
    fn garner<const P: u64>(x1: Fp<P1>, x2: Fp<P2>, x3: Fp<P3>) -> Fp<P> {
        let (x1, x2, x3) = (x1.value(), x2.value(), x3.value());
        let x2 = ((x2 + (P2 - x1)) * mod_inv::<P2>(P1)) % P2;
        let x3 = (((x3 + (P3 - x1)) * mod_inv::<P3>(P1) % P3 + (P3 - x2)) * mod_inv::<P3>(P2)) % P3;
        Fp::new(x1 + P1 * (x2 + P2 * x3 % P))
    }
}
// }}}
