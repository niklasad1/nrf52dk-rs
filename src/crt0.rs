// Copy constants, static, etc to the `data segment`
pub unsafe fn init_data(mut edata: *mut u32, mut sdata: *mut u32, sdata_end: *mut u32) {
    while sdata < sdata_end {
        sdata.write(edata.read());
        sdata = sdata.offset(1);
        edata = edata.offset(1);
    }
}

// Clear the `bss segment` (non initialized data)
pub unsafe fn clear_bss(mut bdata: *mut u32, bdata_end: *mut u32) {
    while bdata < bdata_end {
        bdata.write_volatile(0);
        bdata = bdata.offset(1);
    }
}
