use paste::paste;
use proc_macro2::Literal;

macro_rules! fixed_to_literal {
    ($int_bits:expr, $frac_bits:expr, $signed:expr, $s:expr, $w:expr, $i:expr, $f:expr) => {
            if ($int_bits, $frac_bits, $signed) == ($i, $f, true) {
                return paste![<fixed::types::[<I $i F $f>] as std::str::FromStr>::from_str]($s)
                    .map(|x| x.to_bits()).map(paste![Literal::[<i $w _unsuffixed>]])
            } else if ($int_bits, $frac_bits, $signed) == ($i, $f, false) {
                return paste![<fixed::types::[<U $i F $f>] as std::str::FromStr>::from_str]($s)
                    .map(|x| x.to_bits()).map(paste![Literal::[<u $w _unsuffixed>]])
            }
    };
}

pub fn fixed_to_literal(
    int_bits: u8,
    frac_bits: u8,
    signed: bool,
    s: &str,
) -> Result<Literal, fixed::ParseFixedError> {
    // 8-bit
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 8, 0);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 7, 1);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 6, 2);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 5, 3);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 4, 4);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 3, 5);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 2, 6);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 1, 7);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 8, 0, 8);

    // 16-bit
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 16, 0);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 15, 1);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 14, 2);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 13, 3);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 12, 4);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 11, 5);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 10, 6);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 9, 7);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 8, 8);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 7, 9);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 6, 10);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 5, 11);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 4, 12);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 3, 13);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 2, 14);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 1, 15);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 16, 0, 16);

    // 32-bit
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 32, 0);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 31, 1);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 30, 2);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 29, 3);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 28, 4);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 27, 5);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 26, 6);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 25, 7);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 24, 8);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 23, 9);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 22, 10);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 21, 11);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 20, 12);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 19, 13);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 18, 14);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 17, 15);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 16, 16);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 15, 17);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 14, 18);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 13, 19);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 12, 20);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 11, 21);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 10, 22);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 9, 23);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 8, 24);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 7, 25);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 6, 26);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 5, 27);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 4, 28);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 3, 29);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 2, 30);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 1, 31);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 32, 0, 32);

    // 64-bit
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 64, 0);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 63, 1);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 62, 2);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 61, 3);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 60, 4);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 59, 5);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 58, 6);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 57, 7);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 56, 8);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 55, 9);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 54, 10);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 53, 11);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 52, 12);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 51, 13);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 50, 14);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 49, 15);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 48, 16);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 47, 17);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 46, 18);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 45, 19);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 44, 20);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 43, 21);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 42, 22);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 41, 23);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 40, 24);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 39, 25);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 38, 26);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 37, 27);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 36, 28);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 35, 29);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 34, 30);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 33, 31);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 32, 32);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 31, 33);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 30, 34);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 29, 35);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 28, 36);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 27, 37);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 26, 38);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 25, 39);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 24, 40);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 23, 41);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 22, 42);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 21, 43);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 20, 44);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 19, 45);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 18, 46);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 17, 47);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 16, 48);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 15, 49);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 14, 50);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 13, 51);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 12, 52);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 11, 53);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 10, 54);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 9, 55);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 8, 56);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 7, 57);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 6, 58);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 5, 59);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 4, 60);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 3, 61);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 2, 62);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 1, 63);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 64, 0, 64);

    // 128-bit
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 128, 0);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 127, 1);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 126, 2);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 125, 3);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 124, 4);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 123, 5);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 122, 6);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 121, 7);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 120, 8);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 119, 9);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 118, 10);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 117, 11);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 116, 12);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 115, 13);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 114, 14);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 113, 15);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 112, 16);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 111, 17);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 110, 18);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 109, 19);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 108, 20);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 107, 21);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 106, 22);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 105, 23);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 104, 24);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 103, 25);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 102, 26);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 101, 27);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 100, 28);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 99, 29);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 98, 30);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 97, 31);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 96, 32);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 95, 33);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 94, 34);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 93, 35);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 92, 36);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 91, 37);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 90, 38);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 89, 39);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 88, 40);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 87, 41);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 86, 42);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 85, 43);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 84, 44);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 83, 45);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 82, 46);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 81, 47);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 80, 48);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 79, 49);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 78, 50);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 77, 51);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 76, 52);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 75, 53);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 74, 54);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 73, 55);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 72, 56);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 71, 57);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 70, 58);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 69, 59);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 68, 60);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 67, 61);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 66, 62);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 65, 63);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 64, 64);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 63, 65);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 62, 66);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 61, 67);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 60, 68);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 59, 69);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 58, 70);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 57, 71);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 56, 72);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 55, 73);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 54, 74);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 53, 75);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 52, 76);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 51, 77);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 50, 78);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 49, 79);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 48, 80);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 47, 81);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 46, 82);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 45, 83);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 44, 84);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 43, 85);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 42, 86);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 41, 87);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 40, 88);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 39, 89);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 38, 90);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 37, 91);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 36, 92);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 35, 93);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 34, 94);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 33, 95);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 32, 96);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 31, 97);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 30, 98);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 29, 99);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 28, 100);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 27, 101);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 26, 102);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 25, 103);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 24, 104);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 23, 105);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 22, 106);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 21, 107);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 20, 108);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 19, 109);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 18, 110);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 17, 111);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 16, 112);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 15, 113);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 14, 114);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 13, 115);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 12, 116);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 11, 117);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 10, 118);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 9, 119);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 8, 120);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 7, 121);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 6, 122);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 5, 123);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 4, 124);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 3, 125);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 2, 126);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 1, 127);
    fixed_to_literal!(int_bits, frac_bits, signed, s, 128, 0, 128);

    // shouldn't happen, pinky promise
    unreachable!("no way");
}
