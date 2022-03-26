fn main() {
    let mut dst = [0; 32];
    hex_encode(b"\x01\x02\x03", &mut dst);
    assert_eq!(&dst[..6], b"010203");

    let mut src = [0; 16];
    for i in 0..16 {
        src[i] = (i + 1) as u8;
    }
    hex_encode(&src, &mut dst);
    assert_eq!(&dst, b"0102030405060708090a0b0c0d0e0f10");
}

pub fn hex_encode(src: &[u8], dst: &mut [u8]) {
    let len = src.len().checked_mul(2).unwrap();
    assert!(dst.len() >= len);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse4.1") {
            return unsafe { hex_encode_sse41(src, dst) };
        }
    }

    hex_encode_fallback(src, dst)
}

#[target_feature(enable = "sse4.1")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn hex_encode_sse41(mut src: &[u8], dst: &mut [u8]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // 生成包含16个int8类型的向量，并将全部值设为字符'0'的ascii编号
    let ascii_zero = _mm_set1_epi8(b'0' as i8);
    // 生成包含16个int8类型的向量，并将全部值设为整数9
    let nines = _mm_set1_epi8(9);
    // 生成包含16个int8类型的向量，并将全部值设为字符'a'的ascii编号减去10
    let ascii_a = _mm_set1_epi8((b'a' - 9 - 1) as i8);
    // 生成包含16个int8类型的向量，并将全部值设为二进制数00001111
    let and4bits = _mm_set1_epi8(0xf);

    let mut i = 0_isize;
    while src.len() >= 16 {
        // 从指针中读取128位整数，组成一个128位的向量（可以转化为int8x16、int32x4等形式的向量）
        let invec = _mm_loadu_si128(src.as_ptr() as *const _);
        
        // 将该128位向量类型转化为int8x16类型的向量，并将其中每个元素和二进制数00001111进行与操作
        let masked1 = _mm_and_si128(invec, and4bits);
        // 将该128位向量类型转化为int8x16类型的向量，再将每个元素逻辑右移4位，随后将其中每个元素和二进制数00001111进行与操作
        let masked2 = _mm_and_si128(_mm_srli_epi64(invec, 4), and4bits);

        // 向量对应元素比较大小，获取向量中所有大于9的元素的位置
        let cmpmask1 = _mm_cmpgt_epi8(masked1, nines);
        let cmpmask2 = _mm_cmpgt_epi8(masked2, nines);
        
        // _mm_blendv_epi8表示生成一个新的向量，该向量中的元素是根据cmpmask1中对应位置是否为true选择ascii_zero或者ascii_a中的元素
        // _mm_add_epi8则表示向量对应位置元素相加，结果表示最终生成的十六进制编码的ascii编号
        let masked1 = _mm_add_epi8(
            masked1,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask1),
        );
        let masked2 = _mm_add_epi8(
            masked2,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask2),
        );

        // 生成一个新的向量，其中偶数位置元素（从0开始）来自于masked2，奇数位置元素来自于masked1
        // 该向量共有256位，所以将前128位放入res1中，后128位放入res2中
        let res1 = _mm_unpacklo_epi8(masked2, masked1);
        let res2 = _mm_unpackhi_epi8(masked2, masked1);

        // 将结果向量写入目标指针中
        _mm_storeu_si128(dst.as_mut_ptr().offset(i * 2) as *mut _, res1);
        _mm_storeu_si128(
            dst.as_mut_ptr().offset(i * 2 + 16) as *mut _,
            res2,
        );
        src = &src[16..];
        i += 16;
    }

    let i = i as usize;
    hex_encode_fallback(src, &mut dst[i * 2..]);
}

fn hex_encode_fallback(src: &[u8], dst: &mut [u8]) {
    fn hex(byte: u8) -> u8 {
        static TABLE: &[u8] = b"0123456789abcdef";
        TABLE[byte as usize]
    }

    for (byte, slots) in src.iter().zip(dst.chunks_mut(2)) {
        slots[0] = hex((*byte >> 4) & 0xf);
        slots[1] = hex(*byte & 0xf);
    }
}