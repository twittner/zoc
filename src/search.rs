use crate::{Bbox, Size, GetZ, Z};

/// Iterate over a bounding box of `Z` values.
///
/// Out of a sequence of values that are sorted by their `Z` value, get all
/// elements within a bounding box.
pub fn range<const D: usize, T, A>(items: &[A], min: [T; D], max: [T; D]) -> Zrange<D, T, A>
where
    T: Size<D>,
    A: GetZ<D, T>
{
    let bbox = Bbox::new(min.into(), max.into());
    Zrange {
        stack: vec![Frame { items, min: bbox.min(), max: bbox.max() }],
        bbox,
        threshold: 10
    }
}

/// Iterator over `Z` values.
pub struct Zrange<'a, const D: usize, T: Size<D>, A> {
    stack: Vec<Frame<'a, D, T, A>>,
    bbox: Bbox<D, T>,
    threshold: usize
}

impl<'a, const D: usize, T: Size<D>, A: GetZ<D, T>> Zrange<'a, D, T, A> {
    /// Set litmax/bigmin optimization threshold (default = 10).
    ///
    /// During range search, litmax and bigmin will only be calculated and used
    /// if the remaining number of elements is larger than the threshold value.
    pub fn optimize_if_gt(mut self, t: usize) -> Self {
        self.threshold = t;
        self
    }
}

struct Frame<'a, const D: usize, T: Size<D>, A> {
    items: &'a [A],
    min: Z<D, T>,
    max: Z<D, T>
}

impl<'a, const D: usize, T: Size<D>, A> Iterator for Zrange<'a, D, T, A>
where
    A: GetZ<D, T>
{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(frame) = self.stack.pop() {
            match frame.items.split_at(frame.items.len() / 2) {
            | (lower, [mid, upper @ ..]) => {
                let midz = *mid.z();
                if midz < frame.min {
                    self.stack.push(Frame { items: upper, min: frame.min, max: frame.max })
                } else if midz > frame.max {
                    self.stack.push(Frame { items: lower, min: frame.min, max: frame.max })
                } else if self.bbox.contains(&midz) {
                    self.stack.push(Frame { items: upper, min: midz, max: frame.max });
                    self.stack.push(Frame { items: lower, min: frame.min, max: midz });
                    return Some(mid)
                } else {
                    if upper.len() > self.threshold {
                        let bigmin = self.bbox.bigmin(&midz);
                        self.stack.push(Frame { items: upper, min: bigmin, max: frame.max })
                    } else {
                        self.stack.push(Frame { items: upper, min: midz, max: frame.max })
                    }
                    if lower.len() > self.threshold {
                        let litmax = self.bbox.litmax(&midz);
                        self.stack.push(Frame { items: lower, min: frame.min, max: litmax })
                    } else {
                        self.stack.push(Frame { items: lower, min: frame.min, max: midz })
                    }
                }
            }
            | ([], []) => continue,
            | _        => unreachable!()
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Z;

    #[test]
    fn area() {
        let mut vec = Vec::new();
        for x in 0 .. 9u8 {
            for y in 0 .. 17u8 {
                vec.push(Z::from([y, x]));
            }
        }
        vec.sort_unstable();

        let mut res = (0 .. 129).map(Z::new).collect::<Vec<_>>();
        res.extend([
            129, 132, 133, 144, 145, 148, 149, 192, 193, 196, 197, 208,
            209, 212, 213, 256, 258, 264, 266, 288, 290, 296, 298, 384
        ].map(Z::new));

        assert_eq!(&vec, &res);

        let min = [5,  3];
        let max = [10, 5];

        let mut res = Vec::new();
        for z in super::range(vec.as_slice(), min, max) {
            res.push(z.point);
        }
        res.sort_unstable();
        assert_eq! {
            &[27, 30, 31, 49, 51, 52, 53, 54, 55, 74, 75, 78, 96, 97, 98, 99, 100, 102],
            res.as_slice()
        }
    }
}
