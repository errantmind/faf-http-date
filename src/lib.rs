#![no_std]

#[repr(C)]
struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

const CLOCK_REALTIME: i32 = 0;

// Format: b"Date: Thu, 01 Jan 1970 00:00:00 GMT"
#[inline]
pub fn get_date_buff_with_key() -> [u8; 35] {
    [
        b'D', b'a', b't', b'e', b':', b' ', b' ', b' ', b' ', b',', b' ', b'0', b'0', b' ', b' ',
        b' ', b' ', b' ', b'0', b'0', b'0', b'0', b' ', b'0', b'0', b':', b'0', b'0', b':', b'0',
        b'0', b' ', b'G', b'M', b'T',
    ]
}

// Format: b"Thu, 01 Jan 1970 00:00:00 GMT"
#[inline]
pub fn get_date_buff_no_key() -> [u8; 29] {
    [
        b' ', b' ', b' ', b',', b' ', b'0', b'0', b' ', b' ', b' ', b' ', b' ', b'0', b'0', b'0',
        b'0', b' ', b'0', b'0', b':', b'0', b'0', b':', b'0', b'0', b' ', b'G', b'M', b'T',
    ]
}

// Format: b"Date: Thu, 01 Jan 1970 00:00:00 GMT"
#[inline]
pub fn get_date_with_key(buf: &mut [u8; 35]) {
    let mut ts: timespec = unsafe { core::mem::zeroed() };
    unsafe { clock_gettime(CLOCK_REALTIME, &mut ts as *mut timespec) };

    let secs_since_epoch = ts.tv_sec;

    const LEAPOCH: i64 = 11017;
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;

    let days = (secs_since_epoch / 86400) - LEAPOCH;
    let secs_of_day = secs_since_epoch % 86400;

    let mut qc_cycles = days / DAYS_PER_400Y;
    let mut remdays = days % DAYS_PER_400Y;

    if remdays < 0 {
        remdays += DAYS_PER_400Y;
        qc_cycles -= 1;
    }

    let mut c_cycles = remdays / DAYS_PER_100Y;
    if c_cycles == 4 {
        c_cycles -= 1;
    }
    remdays -= c_cycles * DAYS_PER_100Y;

    let mut q_cycles = remdays / DAYS_PER_4Y;
    if q_cycles == 25 {
        q_cycles -= 1;
    }
    remdays -= q_cycles * DAYS_PER_4Y;

    let mut remyears = remdays / 365;
    if remyears == 4 {
        remyears -= 1;
    }
    remdays -= remyears * 365;

    let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

    let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
    let mut mon = 0;
    for mon_len in months.iter() {
        mon += 1;
        if remdays < *mon_len {
            break;
        }
        remdays -= *mon_len;
    }
    let mday = remdays + 1;
    let mon = if mon + 2 > 12 {
        year += 1;
        mon - 10
    } else {
        mon + 2
    };

    let mut wday = (3 + days) % 7;
    if wday <= 0 {
        wday += 7
    };

    let sec = (secs_of_day % 60) as u8;
    let min = ((secs_of_day % 3600) / 60) as u8;
    let hour = (secs_of_day / 3600) as u8;
    let day = mday as u8;
    let mon = mon as u8;
    let year = year as u16;
    let wday = wday as u8;

    let wday = match wday {
        1 => b"Mon",
        2 => b"Tue",
        3 => b"Wed",
        4 => b"Thu",
        5 => b"Fri",
        6 => b"Sat",
        7 => b"Sun",
        _ => unsafe { core::hint::unreachable_unchecked() },
    };
    let mon = match mon {
        1 => b"Jan",
        2 => b"Feb",
        3 => b"Mar",
        4 => b"Apr",
        5 => b"May",
        6 => b"Jun",
        7 => b"Jul",
        8 => b"Aug",
        9 => b"Sep",
        10 => b"Oct",
        11 => b"Nov",
        12 => b"Dec",
        _ => unsafe { core::hint::unreachable_unchecked() },
    };

    buf[6] = wday[0];
    buf[7] = wday[1];
    buf[8] = wday[2];
    buf[11] = b'0' + (day / 10) as u8;
    buf[12] = b'0' + (day % 10) as u8;
    buf[14] = mon[0];
    buf[15] = mon[1];
    buf[16] = mon[2];
    buf[18] = b'0' + (year / 1000) as u8;
    buf[19] = b'0' + (year / 100 % 10) as u8;
    buf[20] = b'0' + (year / 10 % 10) as u8;
    buf[21] = b'0' + (year % 10) as u8;
    buf[23] = b'0' + (hour / 10) as u8;
    buf[24] = b'0' + (hour % 10) as u8;
    buf[26] = b'0' + (min / 10) as u8;
    buf[27] = b'0' + (min % 10) as u8;
    buf[29] = b'0' + (sec / 10) as u8;
    buf[30] = b'0' + (sec % 10) as u8;
}

// Format: b"Thu, 01 Jan 1970 00:00:00 GMT"
#[inline]
pub fn get_date_no_key(buf: &mut [u8; 29]) {
    let mut ts: timespec = unsafe { core::mem::zeroed() };
    unsafe { clock_gettime(CLOCK_REALTIME, &mut ts as *mut timespec) };

    let secs_since_epoch = ts.tv_sec;

    const LEAPOCH: i64 = 11017;
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;

    let days = (secs_since_epoch / 86400) - LEAPOCH;
    let secs_of_day = secs_since_epoch % 86400;

    let mut qc_cycles = days / DAYS_PER_400Y;
    let mut remdays = days % DAYS_PER_400Y;

    if remdays < 0 {
        remdays += DAYS_PER_400Y;
        qc_cycles -= 1;
    }

    let mut c_cycles = remdays / DAYS_PER_100Y;
    if c_cycles == 4 {
        c_cycles -= 1;
    }
    remdays -= c_cycles * DAYS_PER_100Y;

    let mut q_cycles = remdays / DAYS_PER_4Y;
    if q_cycles == 25 {
        q_cycles -= 1;
    }
    remdays -= q_cycles * DAYS_PER_4Y;

    let mut remyears = remdays / 365;
    if remyears == 4 {
        remyears -= 1;
    }
    remdays -= remyears * 365;

    let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

    let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
    let mut mon = 0;
    for mon_len in months.iter() {
        mon += 1;
        if remdays < *mon_len {
            break;
        }
        remdays -= *mon_len;
    }
    let mday = remdays + 1;
    let mon = if mon + 2 > 12 {
        year += 1;
        mon - 10
    } else {
        mon + 2
    };

    let mut wday = (3 + days) % 7;
    if wday <= 0 {
        wday += 7
    };

    let sec = (secs_of_day % 60) as u8;
    let min = ((secs_of_day % 3600) / 60) as u8;
    let hour = (secs_of_day / 3600) as u8;
    let day = mday as u8;
    let mon = mon as u8;
    let year = year as u16;
    let wday = wday as u8;

    let wday = match wday {
        1 => b"Mon",
        2 => b"Tue",
        3 => b"Wed",
        4 => b"Thu",
        5 => b"Fri",
        6 => b"Sat",
        7 => b"Sun",
        _ => unsafe { core::hint::unreachable_unchecked() },
    };
    let mon = match mon {
        1 => b"Jan",
        2 => b"Feb",
        3 => b"Mar",
        4 => b"Apr",
        5 => b"May",
        6 => b"Jun",
        7 => b"Jul",
        8 => b"Aug",
        9 => b"Sep",
        10 => b"Oct",
        11 => b"Nov",
        12 => b"Dec",
        _ => unsafe { core::hint::unreachable_unchecked() },
    };

    buf[0] = wday[0];
    buf[1] = wday[1];
    buf[2] = wday[2];
    buf[5] = b'0' + (day / 10) as u8;
    buf[6] = b'0' + (day % 10) as u8;
    buf[8] = mon[0];
    buf[9] = mon[1];
    buf[10] = mon[2];
    buf[12] = b'0' + (year / 1000) as u8;
    buf[13] = b'0' + (year / 100 % 10) as u8;
    buf[14] = b'0' + (year / 10 % 10) as u8;
    buf[15] = b'0' + (year % 10) as u8;
    buf[17] = b'0' + (hour / 10) as u8;
    buf[18] = b'0' + (hour % 10) as u8;
    buf[20] = b'0' + (min / 10) as u8;
    buf[21] = b'0' + (min % 10) as u8;
    buf[23] = b'0' + (sec / 10) as u8;
    buf[24] = b'0' + (sec % 10) as u8;
}

// EXAMPLE USAGE
// fn main() {
//     let mut buf = get_date_buff_with_key();
//     get_date_with_key(&mut buf);

//     // Optional, convert to str
//     let date_str = unsafe { std::str::from_utf8_unchecked(&buf[..]) };
//     println!("{}", date_str);

//     let mut buf = get_date_buff_no_key();
//     get_date_no_key(&mut buf);

//     // Optional, convert to str
//     let date_str = unsafe { std::str::from_utf8_unchecked(&buf[..]) };
//     println!("{}", date_str);
// }
