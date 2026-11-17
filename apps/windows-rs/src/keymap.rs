pub const VK_ESCAPE: u32 = 0x1B;
pub const VK_K: u32 = 0x4B;
pub const VK_SHIFT: u32 = 0x10;
pub const VK_MENU: u32 = 0x12;
pub const VK_LSHIFT: u32 = 0xA0;
pub const VK_RSHIFT: u32 = 0xA1;
pub const VK_LMENU: u32 = 0xA4;
pub const VK_RMENU: u32 = 0xA5;

/// The key the page shows for a virtual-key code: letters and digits only.
pub fn to_page_key(vk: u32) -> Option<char> {
    match vk {
        0x41..=0x5A | 0x30..=0x39 => char::from_u32(vk),
        _ => None,
    }
}

pub fn is_alt(vk: u32) -> bool {
    matches!(vk, VK_MENU | VK_LMENU | VK_RMENU)
}

pub fn is_shift(vk: u32) -> bool {
    matches!(vk, VK_SHIFT | VK_LSHIFT | VK_RSHIFT)
}

pub fn is_modifier(vk: u32) -> bool {
    is_alt(vk) || is_shift(vk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_a() {
        assert_eq!(to_page_key(0x41), Some('A'));
    }

    #[test]
    fn maps_z() {
        assert_eq!(to_page_key(0x5A), Some('Z'));
    }

    #[test]
    fn maps_0() {
        assert_eq!(to_page_key(0x30), Some('0'));
    }

    #[test]
    fn maps_9() {
        assert_eq!(to_page_key(0x39), Some('9'));
    }

    #[test]
    fn escape_has_no_page_key() {
        assert_eq!(to_page_key(0x1B), None);
    }

    #[test]
    fn windows_key_has_no_page_key() {
        assert_eq!(to_page_key(0x5B), None);
    }
}
