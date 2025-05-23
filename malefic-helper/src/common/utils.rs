pub unsafe fn pointer_add<T>(base_point: *const T, offset: usize) -> *const core::ffi::c_void {
    ((base_point as *const core::ffi::c_void) as usize + offset) as _
}

pub unsafe fn pointer_sub<T>(base_point: *const T, offset: usize) -> *const core::ffi::c_void {
    ((base_point as *const core::ffi::c_void) as usize - offset) as _
}

pub fn dbj2_str_hash(buffer: &[u8]) -> u32 {
    let mut hsh: u32 = 5381;
    let mut iter: usize = 0;
    let mut cur: u8;

    while iter.lt(&buffer.len()) {
        cur = buffer[iter];

        if cur.eq(&0) {
            iter += 1;
            continue;
        }

        if cur.ge(&('a' as u8)) {
            cur -= 0x20;
        }

        hsh = ((hsh << 5).wrapping_add(hsh)) + cur as u32;
        iter += 1;
    }

    return hsh;
}

pub fn get_cstr_len(pointer: *const u8) -> usize {
    unsafe {
        (0..)
            .take_while(|&i| *(((pointer as usize) + i) as *const u8) != 0)
            .count()
    }
}

pub fn get_wcstr_len(pointer: *const u16) -> usize {
    unsafe {
        (0..)
            .take_while(|&i| *(((pointer as usize) + i) as *const u16) != 0)
            .count()
    }
}

pub fn get_wcc_str_len(pointer: *const u16) -> usize {
    unsafe { (0..).take_while(|&i| *pointer.add(i) != 0).count() }
}

pub fn format_cmdline(processname: String, params: Vec<String>) -> String {
    if params.is_empty() {
        return format_osstring(processname);
    }

    let param_str = params.join(" ");
    format_osstring(processname + " " + &param_str)
}

pub fn format_osstring(os_string: String) -> String {
    use std::ffi::OsString;
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        String::from_utf16_lossy(
            &OsString::from(os_string)
                .encode_wide()
                .collect::<Vec<u16>>(),
        )
    }
    #[cfg(not(target_os = "windows"))]
    {
        String::from(OsString::from(os_string).to_string_lossy())
    }
}

pub fn convert_u8p2vec(data: *const u8) -> Vec<u8> {
    unsafe {
        let mut ret_vec = Vec::new();
        if data.is_null() {
            return ret_vec;
        }
        let mut i = 0;
        loop {
            let byte = *data.add(i);
            if byte == 0 {
                break;
            }
            ret_vec.push(byte);
            i += 1;
        }
        ret_vec
    }
}
