#[cfg(test)]
mod tests {
    use std::i128;

    use biscuit_converter::Biscuit;
    use biscuit_converter::error::ParseIntErr;
    use anyhow::Result;
    const I128_LENGTH_BOUND: usize = 40;  // Adjusted for i128, including sign

    #[test]
    fn test_back_and_forth() -> Result<()> {
        for i in -1_000_000..=1_000_000 {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = i128::parse_decimal(x_byte);
            assert_eq!(
                val, Ok(i),
                "Failed for {}", i
            );
            
        }
        Ok(())
    }

    #[test]
    fn test_to_i128_decimal() -> Result<()> {
        // Test empty input
        let empty: &[u8] = &[];
        assert_eq!(i128::parse_decimal(empty), Err(ParseIntErr::Empty), "Failed for empty input");

        // Test single zero
        let single_zero: &[u8] = &[b'0'];
        assert_eq!(i128::parse_decimal(single_zero), Ok(0), "Failed for single zero");

        for i in 1..I128_LENGTH_BOUND {
            let x_vec: Vec<u8> = vec![b'0'; i];
            let x: &[u8] = &x_vec[..];
            let val = i128::parse_decimal(x);
            assert_eq!(
                val, Ok(0),
                "Failed for {} bytes", i
            );
        }
        for i in 2..I128_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'1'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = i128::parse_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i128>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(I128_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = i128::parse_decimal(x).unwrap();
            let expected: i128 = std::str::from_utf8(x)?.parse::<u128>()? as i128;
            assert_eq!(
                val, expected,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i128_extremes() -> Result<()> {
        // Test values near i128::MAX
        let near_max_byte: &[u8] = b"170141183460469231731687303715884105726";
        let val = i128::parse_decimal(near_max_byte);
        assert_eq!(val, Ok(i128::MAX - 1), "Failed for i128::MAX - 1");

        let max_byte: &[u8] = b"170141183460469231731687303715884105727";
        let val = i128::parse_decimal(max_byte);
        assert_eq!(val, Ok(i128::MAX), "Failed for i128::MAX");

        // Test wrapping behavior at i128::MAX + 1
        let byte_test_p1 = "170141183460469231731687303715884105728";
        let byte_test_p1 = byte_test_p1.as_bytes();
        let val_p1 = i128::parse_decimal(byte_test_p1);
        assert_eq!(val_p1, Err(ParseIntErr::Overflow), "Unexpected behavior for i128::MAX + 1");

        // Test values slightly above i128::MAX + 2
        let byte_test_p2 = "170141183460469231731687303715884105729";
        let val_p2 = i128::parse_decimal(byte_test_p2.as_bytes());
        assert_eq!(val_p2, Err(ParseIntErr::Overflow), "Unexpected behavior for i128::MAX + 2");

        // Test values near i128::MIN
        let near_min = i128::MIN + 1;
        let near_min_string = near_min.to_string();
        let near_min_byte: &[u8] = near_min_string.as_bytes();
        let val = i128::parse_decimal(near_min_byte);
        assert_eq!(val, Ok(near_min), "Failed for i128::MIN + 1");

        let min_string = i128::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = i128::parse_decimal(min_byte);
        assert_eq!(val, Ok(i128::MIN), "Failed for i128::MIN");

        // Test wrapping behavior at i128::MIN - 1
        let byte_test_n1 = i128::MIN.wrapping_sub(1).to_string();
        let byte_test_n1 = byte_test_n1.as_bytes();
        let val_n1 = i128::parse_decimal(byte_test_n1);
        assert_eq!(val_n1, Ok(i128::MAX), "Unexpected behavior for i128::MIN - 1");
        
        // Test u128::MAX (should be interpreted as -1 in two's complement)
        let u128_max_string = u128::MAX.to_string();
        let u128_max_byte: &[u8] = u128_max_string.as_bytes();
        let val = i128::parse_decimal(u128_max_byte);
        assert_eq!(val, Err(ParseIntErr::Overflow), "Failed for u128::MAX");
        
        Ok(())
    }
    
    #[test]
    fn test_i128_leading_zeros() -> Result<()> {
        let byte_leading_zeros_pos = b"0000000090123456789012345678901234567890";
        let byte_leading_zeros_neg = b"-00000000090123456789012345678901234567890";
        let val_leading_zeros_pos = i128::parse_decimal(byte_leading_zeros_pos);
        let val_leading_zeros_neg = i128::parse_decimal(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Ok(90123456789012345678901234567890));
        assert_eq!(val_leading_zeros_neg, Ok(-90123456789012345678901234567890));
        Ok(())
    }

    #[test]
    fn test_base() -> Result<()> {
        let test_vec = vec![
            "-0",
            "0",
            "-1",
            "-12",
            "-123",
            "-1234",
            "-12345",
            "-123456",
            "-1234567",
            "-12345678",
            "-123456789",
            "-1234567890",
            "-12345678901",
            "-123456789012",
            "-1234567890123",
            "-12345678901234",
            "-123456789012345",
            "-1234567890123456",
            "-12345678901234567",
            "-123456789012345678",
            "-1234567890123456789",
            "-12345678901234567890",
            "-123456789012345678901",
            "-1234567890123456789012",
            "-12345678901234567890123",
            "-123456789012345678901234",
            "-1234567890123456789012345",
            "-12345678901234567890123456",
            "-123456789012345678901234567",
            "-1234567890123456789012345678",
            "-12345678901234567890123456789",
            "-123456789012345678901234567890",
            "-1234567890123456789012345678901",
            "-12345678901234567890123456789012",
            "-123456789012345678901234567890123",
            "-1234567890123456789012345678901234",
            "-12345678901234567890123456789012345",
            "-123456789012345678901234567890123456",
            "-1234567890123456789012345678901234567",
            "-12345678901234567890123456789012345678",
        ];
        
        for test in test_vec.iter() {
            let x = test.as_bytes();
            let val = i128::parse_decimal(x);
            assert_eq!(
                val, Ok(test.parse::<i128>()?),
                "Failed for {}, val: {:?}, test {}", test, val, std::str::from_utf8(x)?
            );
        }

        Ok(())
    }
}