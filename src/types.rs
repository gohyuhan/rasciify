#[derive(Clone, Copy)]
pub struct SettingOption {
    pub num_cols: u32,
    pub is_white_bg: bool,
    pub is_color: bool,
}

impl SettingOption {
    pub fn rgb(num_cols: u32) -> SettingOption {
        return SettingOption {
            num_cols,
            is_white_bg: false,
            is_color: true,
        };
    }

    pub fn rgb_white_bg(num_cols: u32) -> SettingOption {
        return SettingOption {
            num_cols,
            is_white_bg: true,
            is_color: true,
        };
    }

    pub fn grayscale(num_cols: u32) -> SettingOption {
        return SettingOption {
            num_cols,
            is_white_bg: false,
            is_color: false,
        };
    }

    pub fn grayscale_white_bg(num_cols: u32) -> SettingOption {
        return SettingOption {
            num_cols,
            is_white_bg: true,
            is_color: false,
        };
    }
}
