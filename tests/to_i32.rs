#[cfg(test)]
mod tests {
    use biscuit_converter::Biscuit;
    use biscuit_converter::error::ParseIntErr;
    use anyhow::Result;
    const I32_LENGTH_BOUND: usize = 11;  // Adjusted for i32, including sign

    #[test]
    fn test_back_and_forth() -> Result<()> {
         
        for i in (-1_000_000..1_000_000).step_by(1000) {
            let x = i.to_string();
            let x_byte: &[u8] = x.as_bytes();
            let val = i32::parse_decimal(x_byte);
            assert_eq!(
                val, Ok(i),
                "Failed for {} the string: \"{}\" byte: {:?}", i, x, x_byte,
            );
        }
        Ok(())
    }

    #[test]
    fn test_to_i32() -> Result<()> {
         
        for i in 2..I32_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'0'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = i32::parse_decimal(x);
            assert_eq!(
                val, Ok(0),
                "Failed for {}-th attempt, where byte: {:?}", i, x,
            );
        }
        for i in 2..I32_LENGTH_BOUND {
            let mut x_vec: Vec<u8> = vec![b'1'; i];
            x_vec[0] = b'-';  // Test negative numbers
            let x: &[u8] = &x_vec[..];
            let val = i32::parse_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i32>()?,
                "Failed for {} bytes", i
            );
        }
        for i in 1..(I32_LENGTH_BOUND-1) {
            let x_vec: Vec<u8> = vec![b'9'; i];
            let x: &[u8] = &x_vec[..];
            let val = i32::parse_decimal(x).unwrap();
            assert_eq!(
                val, std::str::from_utf8(x)?.parse::<i32>()?,
                "Failed for {} bytes", i
            );
        }
        Ok(())
    }

    #[test]
    fn test_i32_extremes() -> Result<()> {
        // Test i32::MAX
        let max_string = i32::MAX.to_string();
        let max_byte: &[u8] = max_string.as_bytes();
        let val = i32::parse_decimal(max_byte);
        assert_eq!(val, Ok(i32::MAX));
        
        // Test i32::MIN
        let min_string = i32::MIN.to_string();
        let min_byte: &[u8] = min_string.as_bytes();
        let val = i32::parse_decimal(min_byte);
        assert_eq!(val, Ok(i32::MIN));
        
        // Test Overflow
        let byte_test_p1 = b"2147483648";  // i32::MAX + 1
        let byte_test_n1 = b"-2147483649";  // i32::MIN - 1
        let val_p1 = i32::parse_decimal(byte_test_p1);
        let val_n1 = i32::parse_decimal(byte_test_n1);
        assert_eq!(val_p1, Err(ParseIntErr::Overflow));
        assert_eq!(val_n1, Err(ParseIntErr::NegOverflow));
        
        Ok(())
    }

    #[test]
    fn test_i32_leading_zeros() -> Result<()> {
         
        let byte_leading_zeros_pos = b"01234567890";
        let byte_leading_zeros_neg = b"-01234567890";
        let val_leading_zeros_pos = i32::parse_decimal(byte_leading_zeros_pos);
        let val_leading_zeros_neg = i32::parse_decimal(byte_leading_zeros_neg);
        assert_eq!(val_leading_zeros_pos, Ok(1234567890));
        assert_eq!(val_leading_zeros_neg, Ok(-1234567890));
        Ok(())
    }
}