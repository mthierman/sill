use windows::Win32::Foundation::COLORREF;

// https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types

// https://learn.microsoft.com/en-us/windows/win32/winmsg/loword
#[allow(non_snake_case)]
pub fn LOWORD(l: u32) -> u16 {
    l as _
}

// https://learn.microsoft.com/en-us/windows/win32/winmsg/hiword
#[allow(non_snake_case)]
pub fn HIWORD(l: u32) -> u16 {
    (l >> 16) as _
}

// https://learn.microsoft.com/en-us/windows/win32/winmsg/lobyte
#[allow(non_snake_case)]
pub fn LOBYTE(w: u16) -> u8 {
    w as _
}

// https://learn.microsoft.com/en-us/windows/win32/winmsg/hibyte
#[allow(non_snake_case)]
pub fn HIBYTE(w: u16) -> u8 {
    (w >> 8) as _
}

// https://learn.microsoft.com/en-us/windows/win32/gdi/colorref
// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rgb
#[allow(non_snake_case)]
pub fn RGB(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF(u32::from(r) | u32::from(g) << 8 | u32::from(b) << 16)
}

// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getrvalue
#[allow(non_snake_case)]
pub fn GetRValue(rgb: COLORREF) -> u8 {
    LOBYTE(rgb.0 as _)
}

// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getgvalue
#[allow(non_snake_case)]
pub fn GetGValue(rgb: COLORREF) -> u8 {
    LOBYTE((rgb.0 >> 8) as _)
}

// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbvalue
#[allow(non_snake_case)]
pub fn GetBValue(rgb: COLORREF) -> u8 {
    LOBYTE((rgb.0 >> 16) as _)
}
