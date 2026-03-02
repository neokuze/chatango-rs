// flags de chatango, sacados de chatango-lib
// los del room, mensajes, mods, todo eso

use std::fmt;

// flags del room/grupo
#[derive(Debug, Clone, Copy, Default)]
pub struct RoomFlags(pub u32);

impl RoomFlags {
    pub const LIST_TAXONOMY: u32            = 1 << 0;
    pub const NO_ANONS: u32                 = 1 << 2;
    pub const NO_FLAGGING: u32              = 1 << 3;
    pub const NO_COUNTER: u32               = 1 << 4;
    pub const NO_IMAGES: u32                = 1 << 5;
    pub const NO_LINKS: u32                 = 1 << 6;
    pub const NO_VIDEOS: u32                = 1 << 7;
    pub const NO_STYLED_TEXT: u32           = 1 << 8;
    pub const NO_LINKS_CHATANGO: u32        = 1 << 9;
    pub const NO_BROADCAST_MSG_WITH_BW: u32 = 1 << 10;
    pub const RATE_LIMIT_REGIMEON: u32      = 1 << 11;
    pub const CHANNELS_DISABLED: u32        = 1 << 13;
    pub const NLP_SINGLEMSG: u32            = 1 << 14;
    pub const NLP_MSGQUEUE: u32             = 1 << 15;
    pub const BROADCAST_MODE: u32           = 1 << 16;
    pub const CLOSED_IF_NO_MODS: u32        = 1 << 17;
    pub const IS_CLOSED: u32                = 1 << 18;
    pub const SHOW_MOD_ICONS: u32           = 1 << 19;
    pub const MODS_CHOOSE_VISIBILITY: u32   = 1 << 20;
    pub const NLP_NGRAM: u32                = 1 << 21;
    pub const NO_PROXIES: u32               = 1 << 22;
    pub const HAS_XML: u32                  = 1 << 28;
    pub const UNSAFE: u32                   = 1 << 29;

    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }
}

impl fmt::Display for RoomFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        let checks: &[(u32, &str)] = &[
            (Self::LIST_TAXONOMY, "LIST_TAXONOMY"),
            (Self::NO_ANONS, "NO_ANONS"),
            (Self::NO_FLAGGING, "NO_FLAGGING"),
            (Self::NO_COUNTER, "NO_COUNTER"),
            (Self::NO_IMAGES, "NO_IMAGES"),
            (Self::NO_LINKS, "NO_LINKS"),
            (Self::NO_VIDEOS, "NO_VIDEOS"),
            (Self::NO_STYLED_TEXT, "NO_STYLED_TEXT"),
            (Self::NO_LINKS_CHATANGO, "NO_LINKS_CHATANGO"),
            (Self::NO_BROADCAST_MSG_WITH_BW, "NO_BROADCAST_BW"),
            (Self::RATE_LIMIT_REGIMEON, "RATE_LIMIT_ON"),
            (Self::CHANNELS_DISABLED, "CHANNELS_DISABLED"),
            (Self::NLP_SINGLEMSG, "NLP_SINGLEMSG"),
            (Self::NLP_MSGQUEUE, "NLP_MSGQUEUE"),
            (Self::BROADCAST_MODE, "BROADCAST_MODE"),
            (Self::CLOSED_IF_NO_MODS, "CLOSED_IF_NO_MODS"),
            (Self::IS_CLOSED, "IS_CLOSED"),
            (Self::SHOW_MOD_ICONS, "SHOW_MOD_ICONS"),
            (Self::MODS_CHOOSE_VISIBILITY, "MODS_CHOOSE_VISIBILITY"),
            (Self::NLP_NGRAM, "NLP_NGRAM"),
            (Self::NO_PROXIES, "NO_PROXIES"),
            (Self::HAS_XML, "HAS_XML"),
            (Self::UNSAFE, "UNSAFE"),
        ];
        for &(flag, name) in checks {
            if self.has(flag) {
                parts.push(name);
            }
        }
        write!(f, "{}", parts.join(" | "))
    }
}

// permisos de moderador
#[derive(Debug, Clone, Copy, Default)]
pub struct ModFlags(pub u32);

impl ModFlags {
    pub const DELETED: u32               = 1 << 0;
    pub const EDIT_MODS: u32             = 1 << 1;
    pub const EDIT_MOD_VISIBILITY: u32   = 1 << 2;
    pub const EDIT_BW: u32               = 1 << 3;
    pub const EDIT_RESTRICTIONS: u32     = 1 << 4;
    pub const EDIT_GROUP: u32            = 1 << 5;
    pub const SEE_COUNTER: u32           = 1 << 6;
    pub const SEE_MOD_CHANNEL: u32       = 1 << 7;
    pub const SEE_MOD_ACTIONS: u32       = 1 << 8;
    pub const EDIT_NLP: u32              = 1 << 9;
    pub const EDIT_GP_ANNC: u32          = 1 << 10;
    pub const EDIT_ADMINS: u32           = 1 << 11;
    pub const EDIT_SUPERMODS: u32        = 1 << 12;
    pub const NO_SENDING_LIMITATIONS: u32= 1 << 13;
    pub const SEE_IPS: u32               = 1 << 14;
    pub const CLOSE_GROUP: u32           = 1 << 15;
    pub const CAN_BROADCAST: u32         = 1 << 16;
    pub const MOD_ICON_VISIBLE: u32      = 1 << 17;
    pub const IS_STAFF: u32              = 1 << 18;
    pub const STAFF_ICON_VISIBLE: u32    = 1 << 19;
    pub const UNBAN_ALL: u32             = 1 << 20;

    // combinado de admin
    pub const ADMIN: u32 = Self::EDIT_MODS | Self::EDIT_RESTRICTIONS
        | Self::EDIT_GROUP | Self::EDIT_GP_ANNC;

    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }

    pub fn is_admin(&self) -> bool {
        self.0 & Self::ADMIN == Self::ADMIN
    }
}

// flags que vienen con cada mensaje
#[derive(Debug, Clone, Copy, Default)]
pub struct MessageFlags(pub u32);

impl MessageFlags {
    pub const PREMIUM: u32          = 1 << 2;
    pub const BG_ON: u32            = 1 << 3;
    pub const MEDIA_ON: u32         = 1 << 4;
    pub const CENSORED: u32         = 1 << 5;
    pub const SHOW_MOD_ICON: u32    = 1 << 6;
    pub const SHOW_STAFF_ICON: u32  = 1 << 7;
    pub const CHANNEL_RED: u32      = 1 << 8;
    pub const CHANNEL_ORANGE: u32   = 1 << 9;
    pub const CHANNEL_GREEN: u32    = 1 << 10;
    pub const CHANNEL_CYAN: u32     = 1 << 11;
    pub const CHANNEL_BLUE: u32     = 1 << 12;
    pub const CHANNEL_PURPLE: u32   = 1 << 13;
    pub const CHANNEL_PINK: u32     = 1 << 14;
    pub const CHANNEL_MOD: u32      = 1 << 15;

    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }

    // parsea el entero de flags del mensaje
    pub fn parse(raw: u32) -> ParsedMessageFlags {
        let flags = MessageFlags(raw);

        let badge = if flags.has(Self::SHOW_STAFF_ICON) {
            2
        } else if flags.has(Self::SHOW_MOD_ICON) {
            1
        } else {
            0
        };

        let channel_name = if flags.has(Self::CHANNEL_MOD) {
            "mod"
        } else if flags.has(Self::CHANNEL_RED) {
            "red"
        } else if flags.has(Self::CHANNEL_ORANGE) {
            "orange"
        } else if flags.has(Self::CHANNEL_GREEN) {
            "green"
        } else if flags.has(Self::CHANNEL_CYAN) {
            "cyan"
        } else if flags.has(Self::CHANNEL_BLUE) {
            "blue"
        } else if flags.has(Self::CHANNEL_PURPLE) {
            "purple"
        } else if flags.has(Self::CHANNEL_PINK) {
            "pink"
        } else {
            ""
        };

        ParsedMessageFlags {
            flags,
            is_premium: flags.has(Self::PREMIUM),
            has_bg: flags.has(Self::BG_ON),
            media_on: flags.has(Self::MEDIA_ON),
            censored: flags.has(Self::CENSORED),
            badge,
            channel_name: channel_name.to_string(),
        }
    }
}

// resultado de parsear flags de un mensaje
#[derive(Debug, Clone)]
pub struct ParsedMessageFlags {
    pub flags: MessageFlags,
    pub is_premium: bool,
    pub has_bg: bool,
    pub media_on: bool,
    pub censored: bool,
    // 0=nada, 1=mod, 2=staff
    pub badge: u8,
    pub channel_name: String,
}

// fuentes de chatango
#[derive(Debug, Clone, Copy)]
pub enum Font {
    Arial = 0,
    Comic = 1,
    Georgia = 2,
    Handwriting = 3,
    Impact = 4,
    Palatino = 5,
    Papyrus = 6,
    Times = 7,
    Typewriter = 8,
}

impl Font {
    pub fn from_id(id: u8) -> Option<Font> {
        match id {
            0 => Some(Font::Arial),
            1 => Some(Font::Comic),
            2 => Some(Font::Georgia),
            3 => Some(Font::Handwriting),
            4 => Some(Font::Impact),
            5 => Some(Font::Palatino),
            6 => Some(Font::Papyrus),
            7 => Some(Font::Times),
            8 => Some(Font::Typewriter),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Font::Arial => "arial",
            Font::Comic => "comic",
            Font::Georgia => "georgia",
            Font::Handwriting => "handwriting",
            Font::Impact => "impact",
            Font::Palatino => "palatino",
            Font::Papyrus => "papyrus",
            Font::Times => "times",
            Font::Typewriter => "typewriter",
        }
    }
}
