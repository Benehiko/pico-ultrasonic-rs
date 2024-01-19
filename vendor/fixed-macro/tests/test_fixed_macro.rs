use fixed_macro::fixed;

macro_rules! check_fixed {
    ($ty:ident, $repr:expr, $($value:tt)+) => {{
        const X: $ty = fixed!($($value)+: $ty);
        assert_eq!(X, <$ty as std::str::FromStr>::from_str($repr).unwrap());
        assert_eq!(format!("{}", X), $repr);
        const Y: $ty = fixed_macro::types::$ty!($($value)+);
        assert_eq!(X, Y);
    }};
}

#[test]
fn test_fixed_macro() {
    {
        use fixed::types::*;

        check_fixed!(U8F0, "255", 0xfF);
        check_fixed!(U4F4, "11", 0b1011);
        check_fixed!(I7F1, "-63", -0o77);
        check_fixed!(U8F8, "0.113", 1.1_2_3_4_5_6_7e-1);
        check_fixed!(I2F14, "-0.11237", -1.1_2_3_4_5_6_7e-1);
        check_fixed!(I64F64, "-0.11234567", -1.1_2_3_4_5_6_7e-1);
        check_fixed!(I24F40, "-0.0012345678", -0_1.2345_6_78E-3);
        check_fixed!(U16F16, "12300", 0.123e5);
        check_fixed!(I8F24, "-123.456", -123.456);
    }
    {
        // separate check in this block, without any imported symbols here
        const X: fixed::types::I32F32 = fixed_macro::fixed!(6_7.8_9_0_1_2e+3: I32F32);
        assert_eq!(X, fixed::types::I32F32::from_str("67890.12").unwrap());
        assert_eq!(format!("{}", X), "67890.12");
        const Y: fixed::types::I32F32 = fixed_macro::types::I32F32!(67.8__90__12e0_3);
        assert_eq!(X, Y);
    }
}
