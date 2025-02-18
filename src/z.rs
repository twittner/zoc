use core::cmp;
use core::fmt;
use core::mem::size_of;
use num_traits::{zero, PrimInt};
use crate::size::Size;

const F: bool = false;
const T: bool = true;

/// A bounding box in `D` dimensions, each containing a value of type `T`.
#[derive(Clone, PartialEq, Eq)]
pub struct Bbox<const D: usize, T: Size<D>> {
    min: Z<D, T>,
    max: Z<D, T>,
    min_parts: [T; D],
    max_parts: [T; D]
}

impl<const D: usize, T: Size<D>> Bbox<D, T> {
    /// Create a new bounding box.
    ///
    /// The given arguments will be normalised such that `min` contains the
    /// minimum value in every dimension and `max` the respective maximums.
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

    /// Get the bbox minimum z-order curve point.
    pub fn min(&self) -> Z<D, T> {
        self.min
    }

    /// Get the bbox maximum z-order curve point.
    pub fn max(&self) -> Z<D, T> {
        self.max
    }

    /// Borrow the bbox minimum values.
    pub fn min_parts(&self) -> &[T; D] {
        &self.min_parts
    }

    /// Borrow the bbox maximum values.
    pub fn max_parts(&self) -> &[T; D] {
        &self.max_parts
    }

    /// Calculate the little maximum value for the given argument `z`.
    ///
    /// If `z` is inside this bounding box, the little maximum is the greatest
    /// point also within this bounding box that is less than `z`.
    pub fn litmax(&self, z: &Z<D, T>) -> Z<D, T> {
        let mut min = self.min.point;
        let mut max = self.max.point;
        let mut litmax = max;
        let start = cmp::min(z.point.leading_zeros(), cmp::min(min.leading_zeros(), max.leading_zeros()));
        let nbits = 8 * size_of::<T>() * D.next_power_of_two();
        for i in (0 .. nbits - start as usize).rev() {
            match (bit(z.point, i), bit(min, i), bit(max, i)) {
            | (F, F, F) => continue,
            | (F, F, T) => {
                max = del_bit(max, i);
                max = max | ones::<T>(i / D).expand() << (i % D);
            }
            | (F, T, F) => unreachable!("min <= max"),
            | (F, T, T) => break,
            | (T, F, F) => {
                litmax = max;
                break
            }
            | (T, F, T) => {
                litmax = del_bit(max, i);
                litmax = litmax | ones::<T>(i / D).expand() << (i % D);
                min    = set_bit(min, i);
                min    = min & !(ones::<T>(i / D).expand() << (i % D));
            }
            | (T, T, F) => unreachable!("min <= max"),
            | (T, T, T) => continue
            }
        }
        Z::new(litmax)
    }

    /// Calculate the big minimum value for the given argument `z`.
    ///
    /// If `z` is inside this bounding box, the big minimum is the smallest
    /// point also within this bounding box that is greater than `z`.
    pub fn bigmin(&self, z: &Z<D, T>) -> Z<D, T> {
        let mut min = self.min.point;
        let mut max = self.max.point;
        let mut bigmin = min;
        let start = cmp::min(z.point.leading_zeros(), cmp::min(min.leading_zeros(), max.leading_zeros()));
        let nbits = 8 * size_of::<T>() * D.next_power_of_two();
        for i in (0 .. nbits - start as usize).rev() {
            match (bit(z.point, i), bit(min, i), bit(max, i)) {
            | (F, F, F) => continue,
            | (F, F, T) => {
                bigmin = set_bit(min, i);
                bigmin = bigmin & !(ones::<T>(i / D).expand() << (i % D));
                max    = del_bit(max, i);
                max    = max | ones::<T>(i / D).expand() << (i % D);
            }
            | (F, T, F) => unreachable!("min <= max"),
            | (F, T, T) => {
                bigmin = min;
                break
            }
            | (T, F, F) => break,
            | (T, F, T) => {
                min = set_bit(min, i);
                min = min & !(ones::<T>(i / D).expand() << (i % D));
            }
            | (T, T, F) => unreachable!("min <= max"),
            | (T, T, T) => continue
            }
        }
        Z::new(bigmin)
    }

    /// Check if the given `z` is within this bounding box.
    pub fn contains(&self, z: &Z<D, T>) -> bool {
        z.deinterlace().iter()
            .zip(&self.min_parts)
            .zip(&self.max_parts)
            .all(|((z, min), max)| min <= z && z <= max)
    }
}

/// A Z-order curve point.
///
/// The point consists of the interleaved bits from all dimensions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Z<const D: usize, T: Size<D>> {
    pub point: <T as Size<D>>::Output
}

impl<const D: usize, T: Size<D>> Z<D, T> {
    pub fn new(point: <T as Size<D>>::Output) -> Self {
        Self { point }
    }

    /// Compute the Z-order curve point by interleaving the bits of all dimensions.
    pub fn interlace(parts: &[T; D]) -> Self {
        let mut z = zero();
        for (d, n) in parts.iter().enumerate() {
            z = z | (n.expand() << d)
        }
        Self { point: z }
    }

    /// Restore the dimensional values of this Z-order curve point.
    pub fn deinterlace(self) -> [T; D] {
        let mut parts = [zero(); D];
        for (d, n) in parts.iter_mut().enumerate() {
            *n = T::compress(self.point >> d)
        }
        parts
    }
}

/// Create a bit mask with n 1s.
#[inline]
fn ones<T: PrimInt>(n: usize) -> T {
    (T::one() << n) - T::one()
}

/// Check if a bit is set.
#[inline]
fn bit<T: PrimInt>(x: T, i: usize) -> bool {
    x & (T::one() << i) != T::zero()
}

/// Set a bit at some index.
#[inline]
fn set_bit<T: PrimInt>(x: T, i: usize) -> T {
    x | (T::one() << i)
}

/// Clear a bit at some index.
#[inline]
fn del_bit<T: PrimInt>(x: T, i: usize) -> T {
    x & !(T::one() << i)
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

#[cfg(feature = "minicbor")]
use minicbor::encode::{self, Encode, Encoder, Write};
#[cfg(feature = "minicbor")]
use minicbor::decode::{self, Decode, Decoder};

#[cfg(feature = "minicbor")]
impl<const D: usize, T: Size<D>, C> Encode<C> for Z<D, T>
where
    <T as Size<D>>::Output: Encode<C>
{
    fn encode<W>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), encode::Error<W::Error>>
    where
        W: Write
    {
        self.point.encode(e, ctx)
    }
}

#[cfg(feature = "minicbor")]
impl<'b, const D: usize, T: Size<D>, C> Decode<'b, C> for Z<D, T>
where
    <T as Size<D>>::Output: Decode<'b, C>
{
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        Ok(Z {
            point: <T as Size<D>>::Output::decode(d, ctx)?
        })
    }
}

#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};
#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer};

#[cfg(feature = "serde")]
impl<const D: usize, T: Size<D>> Serialize for Z<D, T>
where
    <T as Size<D>>::Output: Serialize
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.point.serialize(s)
    }
}

#[cfg(feature = "serde")]
impl<'de, const D: usize, T: Size<D>> Deserialize<'de> for Z<D, T>
where
    <T as Size<D>>::Output: Deserialize<'de>
{
    fn deserialize<S: Deserializer<'de>>(d: S) -> Result<Self, S::Error> {
        Ok(Z {
            point: <T as Size<D>>::Output::deserialize(d)?
        })
    }
}


#[cfg(test)]
mod tests {
    use arbitrary::{Arbitrary, Unstructured};
    use core::mem::size_of;
    use num_traits::zero;
    use rand::RngCore;
    use crate::Size;
    use super::{bit, del_bit, set_bit, Bbox, F, T, Z};

    fn assert<T: for<'a> Arbitrary<'a>>(label: &str, prop: impl Fn(T) -> bool) {
        let mut data = vec![0u8; size_of::<T>()];
        for _ in 0 .. 10_000 {
            let mut u = Unstructured::new(&data);
            let n = u.arbitrary().unwrap();
            assert!(prop(n), "{label}");
            rand::rng().fill_bytes(&mut data)
        }
    }

    impl<const D: usize, T> Arbitrary<'_> for Z<D, T>
    where
        T: Size<D> + 'static,
        <T as Size<D>>::Output: for<'a> Arbitrary<'a>
    {
        fn arbitrary(u: &mut Unstructured) -> Result<Self, arbitrary::Error> {
            Ok(Z::new(<T as Size<D>>::Output::arbitrary(u)?))
        }
    }

    fn simple_interlace<const D: usize, T: Size<D>>(parts: &[T; D]) -> Z<D, T> {
        let mut z = zero();
        for i in 0 .. 8 * size_of::<T>() {
            for (d, n) in parts.into_iter().enumerate() {
                z = match bit(*n, i) {
                    F => del_bit(z, i * D + d),
                    T => set_bit(z, i * D + d)
                }
            };
        }
        Z { point: z }
    }

    fn simple_deinterlace<const D: usize, T: Size<D>>(z: Z<D, T>) -> [T; D] {
        let mut parts = [zero(); D];
        for i in 0 .. 8 * size_of::<T>() {
            for (d, n) in parts.iter_mut().enumerate() {
                *n = match bit(z.point, i * D + d) {
                    F => del_bit(*n, i),
                    T => set_bit(*n, i)
                }
            }
        }
        parts
    }

    #[test]
    fn interlace() {
        fn assert_interlace<const D: usize, T: Size<D>>(label: &str)
        where
            T: for<'a> Arbitrary<'a>
        {
            assert(label, |parts: [T; D]| {
                let x = simple_interlace(&parts);
                let y = Z::interlace(&parts);
                x == y
            })
        }

        assert_interlace::<2,  u8>("D := 2,  T := u8");
        assert_interlace::<3,  u8>("D := 3,  T := u8");
        assert_interlace::<4,  u8>("D := 4,  T := u8");
        assert_interlace::<5,  u8>("D := 5,  T := u8");
        assert_interlace::<6,  u8>("D := 6,  T := u8");
        assert_interlace::<7,  u8>("D := 7,  T := u8");
        assert_interlace::<8,  u8>("D := 8,  T := u8");
        assert_interlace::<9,  u8>("D := 9,  T := u8");
        assert_interlace::<11, u8>("D := 11, T := u8");
        assert_interlace::<12, u8>("D := 12, T := u8");
        assert_interlace::<13, u8>("D := 13, T := u8");
        assert_interlace::<14, u8>("D := 14, T := u8");
        assert_interlace::<15, u8>("D := 15, T := u8");
        assert_interlace::<16, u8>("D := 16, T := u8");

        assert_interlace::<2, u16>("D := 2, T := u16");
        assert_interlace::<3, u16>("D := 3, T := u16");
        assert_interlace::<4, u16>("D := 4, T := u16");
        assert_interlace::<5, u16>("D := 5, T := u16");
        assert_interlace::<6, u16>("D := 6, T := u16");
        assert_interlace::<7, u16>("D := 7, T := u16");
        assert_interlace::<8, u16>("D := 8, T := u16");

        assert_interlace::<2, u32>("D := 2, T := u32");
        assert_interlace::<3, u32>("D := 3, T := u32");
        assert_interlace::<4, u32>("D := 4, T := u32");

        assert_interlace::<2, u64>("D := 2, T := u64")
    }

    #[test]
    fn deinterlace() {
        fn assert_deinterlace<const D: usize, T: Size<D>>(label: &str)
        where
            T: 'static,
            <T as Size<D>>::Output: for<'a> Arbitrary<'a>
        {
            assert(label, |z: Z<D, T>| {
                let x = simple_deinterlace(z);
                let y = Z::deinterlace(z);
                x == y
            })
        }

        assert_deinterlace::<2,  u8>("D := 2,  T := u8");
        assert_deinterlace::<3,  u8>("D := 3,  T := u8");
        assert_deinterlace::<4,  u8>("D := 4,  T := u8");
        assert_deinterlace::<5,  u8>("D := 5,  T := u8");
        assert_deinterlace::<6,  u8>("D := 6,  T := u8");
        assert_deinterlace::<7,  u8>("D := 7,  T := u8");
        assert_deinterlace::<8,  u8>("D := 8,  T := u8");
        assert_deinterlace::<9,  u8>("D := 9,  T := u8");
        assert_deinterlace::<11, u8>("D := 11, T := u8");
        assert_deinterlace::<12, u8>("D := 12, T := u8");
        assert_deinterlace::<13, u8>("D := 13, T := u8");
        assert_deinterlace::<14, u8>("D := 14, T := u8");
        assert_deinterlace::<15, u8>("D := 15, T := u8");
        assert_deinterlace::<16, u8>("D := 16, T := u8");

        assert_deinterlace::<2, u16>("D := 2, T := u16");
        assert_deinterlace::<3, u16>("D := 3, T := u16");
        assert_deinterlace::<4, u16>("D := 4, T := u16");
        assert_deinterlace::<5, u16>("D := 5, T := u16");
        assert_deinterlace::<6, u16>("D := 6, T := u16");
        assert_deinterlace::<7, u16>("D := 7, T := u16");
        assert_deinterlace::<8, u16>("D := 8, T := u16");

        assert_deinterlace::<2, u32>("D := 2, T := u32");
        assert_deinterlace::<3, u32>("D := 3, T := u32");
        assert_deinterlace::<4, u32>("D := 4, T := u32");

        assert_deinterlace::<2, u64>("D := 2, T := u64")
    }

    #[test]
    fn interlace_deinterlace_identity() {
        fn assert_identity<const D: usize, T: Size<D>>(label: &str)
        where
            T: for<'a> Arbitrary<'a>
        {
            assert(label, |parts1: [T; D]| {
                let parts2 = Z::interlace(&parts1).deinterlace();
                parts1 == parts2
            })
        }

        assert_identity::<2,  u8>("D := 2,  T := u8");
        assert_identity::<3,  u8>("D := 3,  T := u8");
        assert_identity::<4,  u8>("D := 4,  T := u8");
        assert_identity::<5,  u8>("D := 5,  T := u8");
        assert_identity::<6,  u8>("D := 6,  T := u8");
        assert_identity::<7,  u8>("D := 7,  T := u8");
        assert_identity::<8,  u8>("D := 8,  T := u8");
        assert_identity::<9,  u8>("D := 9,  T := u8");
        assert_identity::<11, u8>("D := 11, T := u8");
        assert_identity::<12, u8>("D := 12, T := u8");
        assert_identity::<13, u8>("D := 13, T := u8");
        assert_identity::<14, u8>("D := 14, T := u8");
        assert_identity::<15, u8>("D := 15, T := u8");
        assert_identity::<16, u8>("D := 16, T := u8");

        assert_identity::<2, u16>("D := 2, T := u16");
        assert_identity::<3, u16>("D := 3, T := u16");
        assert_identity::<4, u16>("D := 4, T := u16");
        assert_identity::<5, u16>("D := 5, T := u16");
        assert_identity::<6, u16>("D := 6, T := u16");
        assert_identity::<7, u16>("D := 7, T := u16");
        assert_identity::<8, u16>("D := 8, T := u16");

        assert_identity::<2, u32>("D := 2, T := u32");
        assert_identity::<3, u32>("D := 3, T := u32");
        assert_identity::<4, u32>("D := 4, T := u32");

        assert_identity::<2, u64>("D := 2, T := u64")
    }

    #[test]
    fn from_parts() {
        assert("from_parts", |(x, y, z): (u32, u32, u32)| {
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
        })
    }

    #[test]
    fn litmax() {
        fn assert_litmax<const D: usize, T: Size<D>>(label: &str)
        where
            T: 'static,
            <T as Size<D>>::Output: for<'a> Arbitrary<'a>
        {
            assert(label, |(a, b, c): (Z<D, T>, Z<D, T>, Z<D, T>)| {
                let bbox = Bbox::new(b, c);
                let lmx = bbox.litmax(&a);
                if a > bbox.min && a <= bbox.max {
                    lmx < a && lmx >= bbox.min && lmx <= bbox.max
                } else {
                    lmx >= bbox.min && lmx <= bbox.max
                }
            })
        }

        assert_litmax::<2,  u8>("D := 2,  T := u8");
        assert_litmax::<3,  u8>("D := 3,  T := u8");
        assert_litmax::<4,  u8>("D := 4,  T := u8");
        assert_litmax::<5,  u8>("D := 5,  T := u8");
        assert_litmax::<6,  u8>("D := 6,  T := u8");
        assert_litmax::<7,  u8>("D := 7,  T := u8");
        assert_litmax::<8,  u8>("D := 8,  T := u8");
        assert_litmax::<9,  u8>("D := 9,  T := u8");
        assert_litmax::<11, u8>("D := 11, T := u8");
        assert_litmax::<12, u8>("D := 12, T := u8");
        assert_litmax::<13, u8>("D := 13, T := u8");
        assert_litmax::<14, u8>("D := 14, T := u8");
        assert_litmax::<15, u8>("D := 15, T := u8");
        assert_litmax::<16, u8>("D := 16, T := u8");

        assert_litmax::<2, u16>("D := 2, T := u16");
        assert_litmax::<3, u16>("D := 3, T := u16");
        assert_litmax::<4, u16>("D := 4, T := u16");
        assert_litmax::<5, u16>("D := 5, T := u16");
        assert_litmax::<6, u16>("D := 6, T := u16");
        assert_litmax::<7, u16>("D := 7, T := u16");
        assert_litmax::<8, u16>("D := 8, T := u16");

        assert_litmax::<2, u32>("D := 2, T := u32");
        assert_litmax::<3, u32>("D := 3, T := u32");
        assert_litmax::<4, u32>("D := 4, T := u32");

        assert_litmax::<2, u64>("D := 2, T := u64")

    }

    #[test]
    fn bigmin() {
        fn assert_bigmin<const D: usize, T: Size<D>>(label: &str)
        where
            T: 'static,
            <T as Size<D>>::Output: for<'a> Arbitrary<'a>
        {
            assert(label, |(a, b, c): (Z<D, T>, Z<D, T>, Z<D, T>)| {
                let bbox = Bbox::new(b, c);
                let bmi = bbox.bigmin(&a);
                if a >= bbox.min && a < bbox.max {
                    bmi > a && bmi >= bbox.min && bmi <= bbox.max
                } else {
                    bmi >= bbox.min && bmi <= bbox.max
                }
            })
        }

        assert_bigmin::<2,  u8>("D := 2,  T := u8");
        assert_bigmin::<3,  u8>("D := 3,  T := u8");
        assert_bigmin::<4,  u8>("D := 4,  T := u8");
        assert_bigmin::<5,  u8>("D := 5,  T := u8");
        assert_bigmin::<6,  u8>("D := 6,  T := u8");
        assert_bigmin::<7,  u8>("D := 7,  T := u8");
        assert_bigmin::<8,  u8>("D := 8,  T := u8");
        assert_bigmin::<9,  u8>("D := 9,  T := u8");
        assert_bigmin::<11, u8>("D := 11, T := u8");
        assert_bigmin::<12, u8>("D := 12, T := u8");
        assert_bigmin::<13, u8>("D := 13, T := u8");
        assert_bigmin::<14, u8>("D := 14, T := u8");
        assert_bigmin::<15, u8>("D := 15, T := u8");
        assert_bigmin::<16, u8>("D := 16, T := u8");

        assert_bigmin::<2, u16>("D := 2, T := u16");
        assert_bigmin::<3, u16>("D := 3, T := u16");
        assert_bigmin::<4, u16>("D := 4, T := u16");
        assert_bigmin::<5, u16>("D := 5, T := u16");
        assert_bigmin::<6, u16>("D := 6, T := u16");
        assert_bigmin::<7, u16>("D := 7, T := u16");
        assert_bigmin::<8, u16>("D := 8, T := u16");

        assert_bigmin::<2, u32>("D := 2, T := u32");
        assert_bigmin::<3, u32>("D := 3, T := u32");
        assert_bigmin::<4, u32>("D := 4, T := u32");

        assert_bigmin::<2, u64>("D := 2, T := u64")
    }
}
