use num_traits::PrimInt;

pub trait Size<const D: usize>: PrimInt {
    type Output: PrimInt;
}

impl Size<2>  for u8 { type Output =  u16; }
impl Size<3>  for u8 { type Output =  u32; }
impl Size<4>  for u8 { type Output =  u32; }
impl Size<5>  for u8 { type Output =  u64; }
impl Size<6>  for u8 { type Output =  u64; }
impl Size<7>  for u8 { type Output =  u64; }
impl Size<8>  for u8 { type Output =  u64; }
impl Size<9>  for u8 { type Output = u128; }
impl Size<10> for u8 { type Output = u128; }
impl Size<11> for u8 { type Output = u128; }
impl Size<12> for u8 { type Output = u128; }
impl Size<13> for u8 { type Output = u128; }
impl Size<14> for u8 { type Output = u128; }
impl Size<15> for u8 { type Output = u128; }
impl Size<16> for u8 { type Output = u128; }

impl Size<2> for u16 { type Output =  u32; }
impl Size<3> for u16 { type Output =  u64; }
impl Size<4> for u16 { type Output =  u64; }
impl Size<5> for u16 { type Output = u128; }
impl Size<6> for u16 { type Output = u128; }
impl Size<7> for u16 { type Output = u128; }
impl Size<8> for u16 { type Output = u128; }

impl Size<2> for u32 { type Output =  u64; }
impl Size<3> for u32 { type Output = u128; }
impl Size<4> for u32 { type Output = u128; }

impl Size<2> for u64 { type Output = u128; }
