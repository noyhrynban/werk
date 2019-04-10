use std::os::raw::*;
pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type opus_int16 = int16_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
unsafe fn silk_CLZ32(in32: opus_int32) -> opus_int32 {
    if 0 != in32 {
        32 - (::std::mem::size_of::<c_uint>() as c_ulong as c_int * 8i32
            - (in32 as c_uint).leading_zeros() as opus_int32)
    } else {
        32
    }
}
/* Compute number of bits to right shift the sum of squares of a vector    */
/* of int16s to make it fit in an int32                                    */
#[no_mangle]
pub unsafe fn silk_sum_sqr_shift(
    energy: *mut opus_int32,
    shift: *mut c_int,
    x: *const opus_int16,
    len: c_int,
) {
    let mut i: c_int;
    let mut shft: c_int;
    let mut nrg_tmp: opus_uint32;
    let mut nrg: opus_int32;

    shft = 31 - silk_CLZ32(len);

    nrg = len;
    i = 0;
    while i < len - 1 {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1) as isize) as opus_int32 * *x.offset((i + 1) as isize) as opus_int32)
                as opus_uint32,
        ) as opus_int32 as opus_uint32;
        nrg = (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg = (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32
    }
    shft = silk_max_32(0, shft + 3 - silk_CLZ32(nrg));
    nrg = 0;
    i = 0;
    while i < len - 1 {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1) as isize) as opus_int32 * *x.offset((i + 1) as isize) as opus_int32)
                as opus_uint32,
        ) as opus_int32 as opus_uint32;
        nrg = (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as opus_int32 * *x.offset(i as isize) as opus_int32)
            as opus_uint32;
        nrg = (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32
    }
    *shift = shft;
    *energy = nrg;
}
unsafe extern "C" fn silk_max_32(a: opus_int32, b: opus_int32) -> opus_int32 {
    if a > b {
        a
    } else {
        b
    }
}
