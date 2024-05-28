use core::fmt;
use core::mem::size_of;
use num_traits::{zero, PrimInt};
use crate::size::Size;

const F: bool = false;
const T: bool = true;

#[derive(Clone, PartialEq, Eq)]
pub struct Bbox<const D: usize, T: Size<D>> {
    min: Z<D, T>,
    max: Z<D, T>,
    min_parts: [T; D],
    max_parts: [T; D]
}

impl<const D: usize, T: Size<D>> Bbox<D, T> {
    pub fn new(min: Z<D, T>, max: Z<D, T>) -> Self {
        let mut min_parts = [zero(); D];
        let mut max_parts = [zero(); D];
        for (d, (min, max)) in min.deinterlace()
            .into_iter()
            .zip(max.deinterlace())
            .map(|(min, max)| if min <= max { (min, max) } else { (max, min) })
            .enumerate()
        {
            min_parts[d] = min;
            max_parts[d] = max;
        }
        Self {
            min: Z::interlace(&min_parts),
            max: Z::interlace(&max_parts),
            min_parts,
            max_parts
        }
    }

    pub fn min(&self) -> Z<D, T> {
        self.min
    }

    pub fn max(&self) -> Z<D, T> {
        self.max
    }

    pub fn min_parts(&self) -> &[T; D] {
        &self.min_parts
    }

    pub fn max_parts(&self) -> &[T; D] {
        &self.max_parts
    }

    pub fn litmax(&self, z: &Z<D, T>) -> Z<D, T> {
        let mut min = self.min.point;
        let mut max = self.max.point;
        let mut litmax = max;
        for i in (0 .. 8 * size_of::<T>() * D).rev() {
            match (bit(z.point, i), bit(min, i), bit(max, i)) {
            | (F, F, F) => continue,
            | (F, F, T) => {
                max = with_bit(max, i, F);
                for j in (0 .. i).skip(i % D).step_by(D) {
                    max = with_bit(max, j, T)
                }
            }
            | (F, T, F) => unreachable!("min <= max"),
            | (F, T, T) => break,
            | (T, F, F) => {
                litmax = max;
                break
            }
            | (T, F, T) => {
                litmax = with_bit(max, i, F);
                min    = with_bit(min, i, T);
                for j in (0 .. i).skip(i % D).step_by(D) {
                    litmax = with_bit(litmax, j, T);
                    min    = with_bit(min, j, F)
                }
            }
            | (T, T, F) => unreachable!("min <= max"),
            | (T, T, T) => continue
            }
        }
        Z::new(litmax)
    }

    pub fn bigmin(&self, z: &Z<D, T>) -> Z<D, T> {
        let mut min = self.min.point;
        let mut max = self.max.point;
        let mut bigmin = min;
        for i in (0 .. 8 * size_of::<T>() * D).rev() {
            match (bit(z.point, i), bit(min, i), bit(max, i)) {
            | (F, F, F) => continue,
            | (F, F, T) => {
                bigmin = with_bit(min, i, T);
                max    = with_bit(max, i, F);
                for j in (0 .. i).skip(i % D).step_by(D) {
                    bigmin = with_bit(bigmin, j, F);
                    max    = with_bit(max, j, T)
                }
            }
            | (F, T, F) => unreachable!("min <= max"),
            | (F, T, T) => {
                bigmin = min;
                break
            }
            | (T, F, F) => break,
            | (T, F, T) => {
                min = with_bit(min, i, T);
                for j in (0 .. i).skip(i % D).step_by(D) {
                    min = with_bit(min, j, F)
                }
            }
            | (T, T, F) => unreachable!("min <= max"),
            | (T, T, T) => continue
            }
        }
        Z::new(bigmin)
    }

    pub fn contains(&self, z: &Z<D, T>) -> bool {
        z.deinterlace().iter()
            .zip(&self.min_parts)
            .zip(&self.max_parts)
            .all(|((z, min), max)| min <= z && z <= max)
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Z<const D: usize, T: Size<D>> {
    pub point: <T as Size<D>>::Output
}

impl<const D: usize, T: Size<D>> Z<D, T> {
    pub fn new(point: <T as Size<D>>::Output) -> Self {
        Self { point }
    }

    pub fn interlace(parts: &[T; D]) -> Self {
        let mut z = zero();
        for (d, n) in parts.into_iter().enumerate() {
            z = z | (n.expand() << d)
        }
        Self { point: z }
    }

    pub fn deinterlace(self) -> [T; D] {
        let mut parts = [zero(); D];
        for (d, n) in parts.iter_mut().enumerate() {
            *n = T::compress(self.point >> d)
        }
        parts
    }
}

#[inline]
fn bit<T: PrimInt>(x: T, i: usize) -> bool {
    x & (T::one() << i) != T::zero()
}

#[inline]
fn with_bit<T: PrimInt>(x: T, i: usize, b: bool) -> T {
    match b {
    | true  => x |  (T::one() << i),
    | false => x & !(T::one() << i)
    }
}

impl<const D: usize, T: Size<D>> From<[T; D]> for Z<D, T> {
    fn from(val: [T; D]) -> Self {
        Self::interlace(&val)
    }
}

impl<const D: usize, T: Size<D>> From<&'_ [T; D]> for Z<D, T> {
    fn from(val: &[T; D]) -> Self {
        Self::interlace(val)
    }
}

impl<const D: usize, T: Size<D>> From<Z<D, T>> for [T; D] {
    fn from(val: Z<D, T>) -> Self {
        val.deinterlace()
    }
}

impl<const D: usize, T: Size<D>> fmt::Debug for Z<D, T>
where
    <T as Size<D>>::Output: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.point.fmt(f)
    }
}

impl<const D: usize, T: Size<D>> fmt::Display for Z<D, T>
where
    <T as Size<D>>::Output: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.point.fmt(f)
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::mem::size_of;

    use num_traits::zero;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::RngCore;
    use crate::Size;

    use super::with_bit;
    use super::Z;
    use super::bit;
    use super::Bbox;

    impl<const D: usize, T> Arbitrary for Z<D, T>
    where
        T: Size<D> + 'static,
        <T as Size<D>>::Output: Arbitrary
    {
        fn arbitrary(g: &mut Gen) -> Self {
            Z::new(<T as Size<D>>::Output::arbitrary(g))
        }
    }

    fn simple_interlace<const D: usize, T: Size<D>>(parts: &[T; D]) -> Z<D, T> {
        let mut z = zero();
        for i in 0 .. 8 * size_of::<T>() {
            for (d, n) in parts.into_iter().enumerate() {
                z = with_bit(z, i * D + d, bit(*n, i))
            }
        }
        Z { point: z }
    }

    fn simple_deinterlace<const D: usize, T: Size<D>>(z: Z<D, T>) -> [T; D] {
        let mut parts = [zero(); D];
        for i in 0 .. 8 * size_of::<T>() {
            for (d, n) in parts.iter_mut().enumerate() {
                *n = with_bit(*n, i, bit(z.point, i * D + d))
            }
        }
        parts
    }

    quickcheck! {
        fn interlace_u8_2(a: u8, b: u8) -> bool {
            let x = simple_interlace(&[a, b]);
            let y = Z::interlace(&[a, b]);
            x == y
        }

        fn interlace_u8_3(a: u8, b: u8, c: u8) -> bool {
            let x = simple_interlace(&[a, b, c]);
            let y = Z::interlace(&[a, b, c]);
            x == y
        }

        fn interlace_u8_4(a: u8, b: u8, c: u8, d: u8) -> bool {
            let x = simple_interlace(&[a, b, c, d]);
            let y = Z::interlace(&[a, b, c, d]);
            x == y
        }

        fn interlace_u8_5(a: u8, b: u8, c: u8, d: u8, e: u8) -> bool {
            let x = simple_interlace(&[a, b, c, d, e]);
            let y = Z::interlace(&[a, b, c, d, e]);
            x == y
        }

        fn interlace_u8_6(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f]);
            let y = Z::interlace(&[a, b, c, d, e, f]);
            x == y
        }

        fn interlace_u8_7(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f, g]);
            let y = Z::interlace(&[a, b, c, d, e, f, g]);
            x == y
        }

        fn interlace_u8_8(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f, g, h]);
            let y = Z::interlace(&[a, b, c, d, e, f, g, h]);
            x == y
        }

        fn interlace_u16_2(a: u16, b: u16) -> bool {
            let x = simple_interlace(&[a, b]);
            let y = Z::interlace(&[a, b]);
            x == y
        }

        fn interlace_u16_3(a: u16, b: u16, c: u16) -> bool {
            let x = simple_interlace(&[a, b, c]);
            let y = Z::interlace(&[a, b, c]);
            x == y
        }

        fn interlace_u16_4(a: u16, b: u16, c: u16, d: u16) -> bool {
            let x = simple_interlace(&[a, b, c, d]);
            let y = Z::interlace(&[a, b, c, d]);
            x == y
        }

        fn interlace_u16_5(a: u16, b: u16, c: u16, d: u16, e: u16) -> bool {
            let x = simple_interlace(&[a, b, c, d, e]);
            let y = Z::interlace(&[a, b, c, d, e]);
            x == y
        }

        fn interlace_u16_6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f]);
            let y = Z::interlace(&[a, b, c, d, e, f]);
            x == y
        }

        fn interlace_u16_7(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f, g]);
            let y = Z::interlace(&[a, b, c, d, e, f, g]);
            x == y
        }

        fn interlace_u16_8(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> bool {
            let x = simple_interlace(&[a, b, c, d, e, f, g, h]);
            let y = Z::interlace(&[a, b, c, d, e, f, g, h]);
            x == y
        }

        fn interlace_u32_2(a: u32, b: u32) -> bool {
            let x = simple_interlace(&[a, b]);
            let y = Z::interlace(&[a, b]);
            x == y
        }

        fn interlace_u32_3(a: u32, b: u32, c: u32) -> bool {
            let x = simple_interlace(&[a, b, c]);
            let y = Z::interlace(&[a, b, c]);
            x == y
        }

        fn interlace_u32_4(a: u32, b: u32, c: u32, d: u32) -> bool {
            let x = simple_interlace(&[a, b, c, d]);
            let y = Z::interlace(&[a, b, c, d]);
            x == y
        }

        fn interlace_u64_2(a: u32, b: u32) -> bool {
            let x = simple_interlace(&[a, b]);
            let y = Z::interlace(&[a, b]);
            x == y
        }
    }

    fn interlace_u8<const D: usize>()
    where
        u8: Size<D>,
        <u8 as Size<D>>::Output: Debug
    {
        let mut a = [0u8; D];
        for _ in 0 .. 1000 {
            rand::thread_rng().fill_bytes(&mut a);
            let x = simple_interlace(&a);
            let y = Z::interlace(&a);
            assert_eq!(x, y)
        }
    }

    #[test] fn interlace_u8_09() { interlace_u8::<9>() }
    #[test] fn interlace_u8_10() { interlace_u8::<10>() }
    #[test] fn interlace_u8_11() { interlace_u8::<11>() }
    #[test] fn interlace_u8_12() { interlace_u8::<12>() }
    #[test] fn interlace_u8_13() { interlace_u8::<13>() }
    #[test] fn interlace_u8_14() { interlace_u8::<14>() }
    #[test] fn interlace_u8_15() { interlace_u8::<15>() }
    #[test] fn interlace_u8_16() { interlace_u8::<16>() }

    quickcheck! {
        fn deinterlace_u8_2(z: Z<2, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_3(z: Z<3, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_4(z: Z<4, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_5(z: Z<5, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_6(z: Z<6, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_7(z: Z<7, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_8(z: Z<8, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_9(z: Z<9, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_10(z: Z<10, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_11(z: Z<11, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_12(z: Z<12, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_13(z: Z<13, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_14(z: Z<14, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_15(z: Z<15, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u8_16(z: Z<16, u8>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_2(z: Z<2, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_3(z: Z<3, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_4(z: Z<4, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_5(z: Z<5, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_6(z: Z<6, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_7(z: Z<7, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u16_8(z: Z<8, u16>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u32_2(z: Z<2, u32>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u32_3(z: Z<3, u32>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u32_4(z: Z<4, u32>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }

        fn deinterlace_u64_2(z: Z<2, u64>) -> bool {
            let x = simple_deinterlace(z);
            let y = Z::deinterlace(z);
            x == y
        }
    }

    quickcheck! {
        fn from_parts_to_parts(x1: u32, y1: u32) -> bool {
            let [x2, y2] = Z::interlace(&[x1, y1]).deinterlace();
            x1 == x2 && y1 == y2
        }

        fn from_parts(x: u32, y: u32, z: u32) -> bool {
            let val = Z::interlace(&[x, y, z]);
            for i in 0 .. 32 {
                if bit(val.point, i * 3) != bit(x, i) {
                    return false
                }
                if bit(val.point, i * 3 + 1) != bit(y, i) {
                    return false
                }
                if bit(val.point, i * 3 + 2) != bit(z, i) {
                    return false
                }
            }
            true
        }
    }

    quickcheck! {
        /// Assuming [b,c] and a within range, litmax is the greatest code
        /// within range that is less than a.
        fn litmax(a: Z<2, u8>, b: Z<2, u8>, c: Z<2, u8>) -> bool {
            let bbox = Bbox::new(b, c);
            let lmx = bbox.litmax(&a);
            if a > bbox.min && a <= bbox.max {
                lmx < a && lmx >= bbox.min && lmx <= bbox.max
            } else {
                lmx >= bbox.min && lmx <= bbox.max
            }
        }

        /// Assuming [b,c] and a within range, bigmin is the smallest code
        /// within range that is greater than a.
        fn bigmin(a: Z<2, u8>, b: Z<2, u8>, c: Z<2, u8>) -> bool {
            let bbox = Bbox::new(b, c);
            let lmx = bbox.bigmin(&a);
            if a >= bbox.min && a < bbox.max {
                lmx > a && lmx >= bbox.min && lmx <= bbox.max
            } else {
                lmx >= bbox.min && lmx <= bbox.max
            }
        }
    }
}
