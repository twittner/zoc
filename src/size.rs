use num_traits::PrimInt;

pub trait Size<const D: usize>: PrimInt {
    type Output: PrimInt;

    fn expand(self) -> <Self as Size<D>>::Output;

    fn compress(output: <Self as Size<D>>::Output) -> Self;
}

impl Size<2> for u8 {
    type Output = u16;

    #[inline]
    fn expand(self) -> u16 {
        let mut out = self as u16;
        out = (out ^ (out << 4)) & 0x0F0F;
        out = (out ^ (out << 2)) & 0x3333;
        out = (out ^ (out << 1)) & 0x5555;
        out
    }

    #[inline]
    fn compress(val: u16) -> u8 {
        let mut out = val & 0x5555;
        out = (out ^ (out >> 1)) & 0x3333;
        out = (out ^ (out >> 2)) & 0x0F0F;
        out = (out ^ (out >> 4)) & 0x00FF;
        out as u8
    }
}

impl Size<3> for u8 {
    type Output = u32;

    #[inline]
    fn expand(self) -> u32 {
        let mut out = self as u32;
        out = (out ^ (out <<  8)) & 0x0F00_F00F;
        out = (out ^ (out <<  4)) & 0xC30C_30C3;
        out = (out ^ (out <<  2)) & 0x4924_9249;
        out
    }

    #[inline]
    fn compress(val: u32) -> u8 {
        let mut out = val & 0x4924_9249;
        out = (out ^ (out >> 2)) & 0xC30C_30C3;
        out = (out ^ (out >> 4)) & 0x0F00_F00F;
        out = (out ^ (out >> 8)) & 0xFF00_00FF;
        out as u8
    }
}

impl Size<4> for u8 {
    type Output = u32;

    #[inline]
    fn expand(self) -> u32 {
        let mut out = self as u32;
        out = (out ^ (out << 12)) & 0x000F_000F;
        out = (out ^ (out <<  6)) & 0x0303_0303;
        out = (out ^ (out <<  3)) & 0x1111_1111;
        out
    }

    #[inline]
    fn compress(val: u32) -> u8 {
        let mut out = val & 0x1111_1111;
        out = (out ^ (out >>  3)) & 0x0303_0303;
        out = (out ^ (out >>  6)) & 0x000F_000F;
        out = (out ^ (out >> 12)) & 0x0000_00FF;
        out as u8
    }
}

impl Size<5> for u8 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 16)) & 0xF000_0F00_00F0_000F;
        out = (out ^ (out <<  8)) & 0x300C_0300_C030_0C03;
        out = (out ^ (out <<  4)) & 0x1084_2108_4210_8421;
        out
    }

    #[inline]
    fn compress(val: u64) -> u8 {
        let mut out = val & 0x1084_2108_4210_8421;
        out = (out ^ (out >>  4)) & 0x300C_0300_C030_0C03;
        out = (out ^ (out >>  8)) & 0xF000_0F00_00F0_000F;
        out = (out ^ (out >> 16)) & 0x0000_FF00_0000_00FF;
        out as u8
    }
}

impl Size<6> for u8 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 20)) & 0x000F_0000_0F00_000F;
        out = (out ^ (out << 10)) & 0x3003_0030_0300_3003;
        out = (out ^ (out <<  5)) & 0x1041_0410_4104_1041;
        out
    }

    #[inline]
    fn compress(val: u64) -> u8 {
        let mut out = val & 0x1041_0410_4104_1041;
        out = (out ^ (out >>  5)) & 0x3003_0030_0300_3003;
        out = (out ^ (out >> 10)) & 0x000F_0000_0F00_000F;
        out = (out ^ (out >> 20)) & 0x00FF_0000_0000_00FF;
        out as u8
    }
}

impl Size<7> for u8 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 24)) & 0x0F00_0000_F000_000F;
        out = (out ^ (out << 12)) & 0x0300_0C00_3000_C003;
        out = (out ^ (out <<  6)) & 0x8102_0408_1020_4081;
        out
    }

    #[inline]
    fn compress(val: u64) -> u8 {
        let mut out = val & 0x8102_0408_1020_4081;
        out = (out ^ (out >>  6)) & 0x0300_0C00_3000_C003;
        out = (out ^ (out >> 12)) & 0x0F00_0000_F000_000F;
        out = (out ^ (out >> 24)) & 0xFF00_0000_0000_00FF;
        out as u8
    }
}

impl Size<8> for u8 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 28)) & 0x0000_000F_0000_000F;
        out = (out ^ (out << 14)) & 0x0003_0003_0003_0003;
        out = (out ^ (out <<  7)) & 0x0101_0101_0101_0101;
        out
    }

    #[inline]
    fn compress(val: u64) -> u8 {
        let mut out = val & 0x0101_0101_0101_0101;
        out = (out ^ (out >>  7)) & 0x0003_0003_0003_0003;
        out = (out ^ (out >> 14)) & 0x0000_000F_0000_000F;
        out = (out ^ (out >> 28)) & 0x0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<9> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 32)) & 0x00F0_0000_000F_0000_0000_00F0_0000_000F;
        out = (out ^ (out << 16)) & 0xC000_3000_0C00_0300_00C0_0030_000C_0003;
        out = (out ^ (out <<  8)) & 0x4020_1008_0402_0100_8040_2010_0804_0201;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x4020_1008_0402_0100_8040_2010_0804_0201;
        out = (out ^ (out >>  8)) & 0xC000_3000_0C00_0300_00C0_0030_000C_0003;
        out = (out ^ (out >> 16)) & 0x00F0_0000_000F_0000_0000_00F0_0000_000F;
        out = (out ^ (out >> 32)) & 0x0000_0000_0000_FF00_0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<10> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 36)) & 0x00F0_0000_000F_0000_0000_0F00_0000_000F;
        out = (out ^ (out << 18)) & 0x0300_0030_0003_0000_3000_0300_0030_0003;
        out = (out ^ (out <<  9)) & 0x0100_4010_0401_0040_1004_0100_4010_0401;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0100_4010_0401_0040_1004_0100_4010_0401;
        out = (out ^ (out >>  9)) & 0x0300_0030_0003_0000_3000_0300_0030_0003;
        out = (out ^ (out >> 18)) & 0x00F0_0000_000F_0000_0000_0F00_0000_000F;
        out = (out ^ (out >> 36)) & 0x0000_0000_0000_0000_0000_0000_0000_FFFF;
        out as u8
    }
}

impl Size<11> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 40)) & 0x0000_0000_0F00_0000_0000_F000_0000_000F;
        out = (out ^ (out << 20)) & 0x0000_C000_0300_000C_0000_3000_00C0_0003;
        out = (out ^ (out << 10)) & 0x0200_4008_0100_2004_0080_1002_0040_0801;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0200_4008_0100_2004_0080_1002_0040_0801;
        out = (out ^ (out >> 10)) & 0x0000_C000_0300_000C_0000_3000_00C0_0003;
        out = (out ^ (out >> 20)) & 0x0000_0000_0F00_0000_0000_F000_0000_000F;
        out = (out ^ (out >> 40)) & 0x0000_0000_FF00_0000_0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<12> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 44)) & 0x0000_000F_0000_0000_000F_0000_0000_000F;
        out = (out ^ (out << 22)) & 0x0300_0003_0000_0300_0003_0000_0300_0003;
        out = (out ^ (out << 11)) & 0x0100_1001_0010_0100_1001_0010_0100_1001;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0100_1001_0010_0100_1001_0010_0100_1001;
        out = (out ^ (out >> 11)) & 0x0300_0003_0000_0300_0003_0000_0300_0003;
        out = (out ^ (out >> 22)) & 0x0000_000F_0000_0000_000F_0000_0000_000F;
        out = (out ^ (out >> 44)) & 0x0000_00FF_0000_0000_0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<13> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 48)) & 0x0000_0F00_0000_0000_00F0_0000_0000_000F;
        out = (out ^ (out << 24)) & 0x0000_0300_0000_C000_0030_0000_0C00_0003;
        out = (out ^ (out << 12)) & 0x0020_0100_0800_4002_0010_0080_0400_2001;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0020_0100_0800_4002_0010_0080_0400_2001;
        out = (out ^ (out >> 12)) & 0x0000_0300_0000_C000_0030_0000_0C00_0003;
        out = (out ^ (out >> 24)) & 0x0000_0F00_0000_0000_00F0_0000_0000_000F;
        out = (out ^ (out >> 48)) & 0x0000_FF00_0000_0000_0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<14> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 52)) & 0x000F_0000_0000_0000_0F00_0000_0000_000F;
        out = (out ^ (out << 26)) & 0x0003_0000_0030_0000_0300_0000_3000_0003;
        out = (out ^ (out << 13)) & 0x4001_0004_0010_0040_0100_0400_1000_4001;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x4001_0004_0010_0040_0100_0400_1000_4001;
        out = (out ^ (out >> 13)) & 0x0003_0000_0030_0000_0300_0000_3000_0003;
        out = (out ^ (out >> 26)) & 0x000F_0000_0000_0000_0F00_0000_0000_000F;
        out = (out ^ (out >> 52)) & 0x00FF_0000_0000_0000_0000_0000_0000_00FF;
        out as u8
    }
}

impl Size<15> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 56)) & 0x0F00_0000_0000_0000_F000_0000_0000_000F;
        out = (out ^ (out << 28)) & 0x0300_0000_0C00_0000_3000_0000_C000_0003;
        out = (out ^ (out << 14)) & 0x0100_0200_0400_0800_1000_2000_4000_8001;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0100_0200_0400_0800_1000_2000_4000_8001;
        out = (out ^ (out >> 14)) & 0x0300_0000_0C00_0000_3000_0000_C000_0003;
        out = (out ^ (out >> 28)) & 0x0F00_0000_0000_0000_F000_0000_0000_000F;
        out = (out ^ (out >> 56)) & 0x0000_0000_0000_0000_0000_0000_0000_FFFF;
        out as u8
    }
}

impl Size<16> for u8 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 60)) & 0x0000_0000_0000_000F_0000_0000_0000_000F;
        out = (out ^ (out << 30)) & 0x0000_0003_0000_0003_0000_0003_0000_0003;
        out = (out ^ (out << 15)) & 0x0001_0001_0001_0001_0001_0001_0001_0001;
        out
    }

    #[inline]
    fn compress(val: u128) -> u8 {
        let mut out = val & 0x0001_0001_0001_0001_0001_0001_0001_0001;
        out = (out ^ (out >> 15)) & 0x0000_0003_0000_0003_0000_0003_0000_0003;
        out = (out ^ (out >> 30)) & 0x0000_0000_0000_000F_0000_0000_0000_000F;
        out = (out ^ (out >> 60)) & 0x0000_0000_0000_0000_0000_0000_0000_FFFF;
        out as u8
    }
}

impl Size<2> for u16 {
    type Output = u32;

    #[inline]
    fn expand(self) -> u32 {
        let mut out = self as u32;
        out = (out ^ (out << 8)) & 0x00FF_00FF;
        out = (out ^ (out << 4)) & 0x0F0F_0F0F;
        out = (out ^ (out << 2)) & 0x3333_3333;
        out = (out ^ (out << 1)) & 0x5555_5555;
        out
    }

    #[inline]
    fn compress(val: u32) -> u16 {
        let mut out = val & 0x5555_5555;
        out = (out ^ (out >> 1)) & 0x3333_3333;
        out = (out ^ (out >> 2)) & 0x0F0F_0F0F;
        out = (out ^ (out >> 4)) & 0x00FF_00FF;
        out = (out ^ (out >> 8)) & 0x0000_FFFF;
        out as u16
    }
}

impl Size<3> for u16 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 16)) & 0x00FF_0000_FF00_00FF;
        out = (out ^ (out <<  8)) & 0xF00F_00F0_0F00_F00F;
        out = (out ^ (out <<  4)) & 0x30C3_0C30_C30C_30C3;
        out = (out ^ (out <<  2)) & 0x9249_2492_4924_9249;
        out
    }

    #[inline]
    fn compress(val: u64) -> u16 {
        let mut out = val & 0x9249_2492_4924_9249;
        out = (out ^ (out >>  2)) & 0x30C3_0C30_C30C_30C3;
        out = (out ^ (out >>  4)) & 0xF00F_00F0_0F00_F00F;
        out = (out ^ (out >>  8)) & 0x00FF_0000_FF00_00FF;
        out = (out ^ (out >> 16)) & 0xFFFF_0000_0000_FFFF;
        out as u16
    }
}

impl Size<4> for u16 {
    type Output = u64;

    #[inline]
    fn expand(self) -> <Self as Size<4>>::Output {
        let mut out = self as u64;
        out = (out ^ (out << 24)) & 0x0000_00FF_0000_00FF;
        out = (out ^ (out << 12)) & 0x000F_000F_000F_000F;
        out = (out ^ (out <<  6)) & 0x0303_0303_0303_0303;
        out = (out ^ (out <<  3)) & 0x1111_1111_1111_1111;
        out
    }

    #[inline]
    fn compress(val: u64) -> u16 {
        let mut out = val & 0x1111_1111_1111_1111;
        out = (out ^ (out >>  3)) & 0x0303_0303_0303_0303;
        out = (out ^ (out >>  6)) & 0x000F_000F_000F_000F;
        out = (out ^ (out >> 12)) & 0x0000_00FF_0000_00FF;
        out = (out ^ (out >> 24)) & 0x0000_0000_0000_FFFF;
        out as u16
    }
}

impl Size<5> for u16 {
    type Output = u128;

    #[inline]
    fn expand(self) -> <Self as Size<5>>::Output {
        let mut out = self as u128;
        out = (out ^ (out << 32)) & 0xFF00_0000_00FF_0000_0000_FF00_0000_00FF;
        out = (out ^ (out << 16)) & 0x0F00_00F0_000F_0000_F000_0F00_00F0_000F;
        out = (out ^ (out <<  8)) & 0x0300_C030_0C03_00C0_300C_0300_C030_0C03;
        out = (out ^ (out <<  4)) & 0x2108_4210_8421_0842_1084_2108_4210_8421;
        out
    }

    #[inline]
    fn compress(val: u128) -> u16 {
        let mut out = val & 0x2108_4210_8421_0842_1084_2108_4210_8421;
        out = (out ^ (out >>  4)) & 0x0300_C030_0C03_00C0_300C_0300_C030_0C03;
        out = (out ^ (out >>  8)) & 0x0F00_00F0_000F_0000_F000_0F00_00F0_000F;
        out = (out ^ (out >> 16)) & 0xFF00_0000_00FF_0000_0000_FF00_0000_00FF;
        out = (out ^ (out >> 32)) & 0x0000_0000_0000_FFFF_0000_0000_0000_FFFF;
        out as u16
    }
}

impl Size<6> for u16 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 40)) & 0x0000_00FF_0000_0000_00FF_0000_0000_00FF;
        out = (out ^ (out << 20)) & 0x0F00_000F_0000_0F00_000F_0000_0F00_000F;
        out = (out ^ (out << 10)) & 0x0300_3003_0030_0300_3003_0030_0300_3003;
        out = (out ^ (out <<  5)) & 0x4104_1041_0410_4104_1041_0410_4104_1041;
        out
    }

    #[inline]
    fn compress(val: u128) -> u16 {
        let mut out = val & 0x4104_1041_0410_4104_1041_0410_4104_1041;
        out = (out ^ (out >>  5)) & 0x0300_3003_0030_0300_3003_0030_0300_3003;
        out = (out ^ (out >> 10)) & 0x0F00_000F_0000_0F00_000F_0000_0F00_000F;
        out = (out ^ (out >> 20)) & 0x0000_00FF_0000_0000_00FF_0000_0000_00FF;
        out = (out ^ (out >> 40)) & 0x0000_FFFF_0000_0000_0000_0000_0000_FFFF;
        out as u16
    }
}
impl Size<7> for u16 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 48)) & 0x00FF_0000_0000_0000_FF00_0000_0000_00FF;
        out = (out ^ (out << 24)) & 0x000F_0000_00F0_0000_0F00_0000_F000_000F;
        out = (out ^ (out << 12)) & 0xC003_000C_0030_00C0_0300_0C00_3000_C003;
        out = (out ^ (out <<  6)) & 0x4081_0204_0810_2040_8102_0408_1020_4081;
        out
    }

    #[inline]
    fn compress(val: u128) -> u16 {
        let mut out = val & 0x4081_0204_0810_2040_8102_0408_1020_4081;
        out = (out ^ (out >>  6)) & 0xC003_000C_0030_00C0_0300_0C00_3000_C003;
        out = (out ^ (out >> 12)) & 0x000F_0000_00F0_0000_0F00_0000_F000_000F;
        out = (out ^ (out >> 24)) & 0x00FF_0000_0000_0000_FF00_0000_0000_00FF;
        out = (out ^ (out >> 48)) & 0xFFFF_0000_0000_0000_0000_0000_0000_FFFF;
        out as u16
    }
}

impl Size<8> for u16 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 56)) & 0x0000_0000_0000_00FF_0000_0000_0000_00FF;
        out = (out ^ (out << 28)) & 0x0000_000F_0000_000F_0000_000F_0000_000F;
        out = (out ^ (out << 14)) & 0x0003_0003_0003_0003_0003_0003_0003_0003;
        out = (out ^ (out <<  7)) & 0x0101_0101_0101_0101_0101_0101_0101_0101;
        out
    }

    #[inline]
    fn compress(val: u128) -> u16 {
        let mut out = val & 0x0101_0101_0101_0101_0101_0101_0101_0101;
        out = (out ^ (out >>  7)) & 0x0003_0003_0003_0003_0003_0003_0003_0003;
        out = (out ^ (out >> 14)) & 0x0000_000F_0000_000F_0000_000F_0000_000F;
        out = (out ^ (out >> 28)) & 0x0000_0000_0000_00FF_0000_0000_0000_00FF;
        out = (out ^ (out >> 56)) & 0x0000_0000_0000_0000_0000_0000_0000_FFFF;
        out as u16
    }
}

impl Size<2> for u32 {
    type Output = u64;

    #[inline]
    fn expand(self) -> u64 {
        let mut out = self as u64;
        out = (out ^ (out << 16)) & 0x0000_FFFF_0000_FFFF;
        out = (out ^ (out <<  8)) & 0x00FF_00FF_00FF_00FF;
        out = (out ^ (out <<  4)) & 0x0F0F_0F0F_0F0F_0F0F;
        out = (out ^ (out <<  2)) & 0x3333_3333_3333_3333;
        out = (out ^ (out <<  1)) & 0x5555_5555_5555_5555;
        out
    }

    #[inline]
    fn compress(val: u64) -> u32 {
        let mut out = val & 0x5555_5555_5555_5555;
        out = (out ^ (out >>  1)) & 0x3333_3333_3333_3333;
        out = (out ^ (out >>  2)) & 0x0F0F_0F0F_0F0F_0F0F;
        out = (out ^ (out >>  4)) & 0x00FF_00FF_00FF_00FF;
        out = (out ^ (out >>  8)) & 0x0000_FFFF_0000_FFFF;
        out = (out ^ (out >> 16)) & 0x0000_0000_FFFF_FFFF;
        out as u32
    }
}

impl Size<3> for u32 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 32)) & 0x0000_FFFF_0000_0000_FFFF_0000_0000_FFFF;
        out = (out ^ (out << 16)) & 0xFF00_00FF_0000_FF00_00FF_0000_FF00_00FF;
        out = (out ^ (out <<  8)) & 0x0F00_F00F_00F0_0F00_F00F_00F0_0F00_F00F;
        out = (out ^ (out <<  4)) & 0xC30C_30C3_0C30_C30C_30C3_0C30_C30C_30C3;
        out = (out ^ (out <<  2)) & 0x4924_9249_2492_4924_9249_2492_4924_9249;
        out
    }

    #[inline]
    fn compress(val: u128) -> u32 {
        let mut out = val & 0x4924_9249_2492_4924_9249_2492_4924_9249;
        out = (out ^ (out >>  2)) & 0xC30C_30C3_0C30_C30C_30C3_0C30_C30C_30C3;
        out = (out ^ (out >>  4)) & 0x0F00_F00F_00F0_0F00_F00F_00F0_0F00_F00F;
        out = (out ^ (out >>  8)) & 0xFF00_00FF_0000_FF00_00FF_0000_FF00_00FF;
        out = (out ^ (out >> 16)) & 0x0000_FFFF_0000_0000_FFFF_0000_0000_FFFF;
        out = (out ^ (out >> 32)) & 0xFFFF_FFFF_0000_0000_0000_0000_FFFF_FFFF;
        out as u32
    }
}

impl Size<4> for u32 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 48)) & 0x0000_0000_0000_FFFF_0000_0000_0000_FFFF;
        out = (out ^ (out << 24)) & 0x0000_00FF_0000_00FF_0000_00FF_0000_00FF;
        out = (out ^ (out << 12)) & 0x000F_000F_000F_000F_000F_000F_000F_000F;
        out = (out ^ (out <<  6)) & 0x0303_0303_0303_0303_0303_0303_0303_0303;
        out = (out ^ (out <<  3)) & 0x1111_1111_1111_1111_1111_1111_1111_1111;
        out
    }

    #[inline]
    fn compress(val: u128) -> u32 {
        let mut out = val & 0x1111_1111_1111_1111_1111_1111_1111_1111;
        out = (out ^ (out >>  3)) & 0x0303_0303_0303_0303_0303_0303_0303_0303;
        out = (out ^ (out >>  6)) & 0x000F_000F_000F_000F_000F_000F_000F_000F;
        out = (out ^ (out >> 12)) & 0x0000_00FF_0000_00FF_0000_00FF_0000_00FF;
        out = (out ^ (out >> 24)) & 0x0000_0000_0000_FFFF_0000_0000_0000_FFFF;
        out = (out ^ (out >> 48)) & 0x0000_0000_0000_0000_0000_0000_FFFF_FFFF;
        out as u32
    }
}

impl Size<2> for u64 {
    type Output = u128;

    #[inline]
    fn expand(self) -> u128 {
        let mut out = self as u128;
        out = (out ^ (out << 32)) & 0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF;
        out = (out ^ (out << 16)) & 0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF;
        out = (out ^ (out <<  8)) & 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF;
        out = (out ^ (out <<  4)) & 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F;
        out = (out ^ (out <<  2)) & 0x3333_3333_3333_3333_3333_3333_3333_3333;
        out = (out ^ (out <<  1)) & 0x5555_5555_5555_5555_5555_5555_5555_5555;
        out
    }

    #[inline]
    fn compress(val: u128) -> u64 {
        let mut out = val & 0x5555_5555_5555_5555_5555_5555_5555_5555;
        out = (out ^ (out >>  1)) & 0x3333_3333_3333_3333_3333_3333_3333_3333;
        out = (out ^ (out >>  2)) & 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F;
        out = (out ^ (out >>  4)) & 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF;
        out = (out ^ (out >>  8)) & 0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF;
        out = (out ^ (out >> 16)) & 0x0000_0000_FFFF_FFFF_0000_0000_FFFF_FFFF;
        out = (out ^ (out >> 32)) & 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF;
        out as u64
    }
}
