/// Icons used throughout several components.
pub struct Icons {
    pub added: &'static str,
    pub changed: &'static str,
    pub deprecated: &'static str,
    pub removed: &'static str,
    pub fixed: &'static str,
    pub security: &'static str,
    pub tag: &'static str,
    pub calendar: &'static str,
}

impl Icons {
    pub const fn nerdfont() -> Self {
        Icons {
            added: "\u{eadc}",
            changed: "\u{eafd}",
            deprecated: "\u{ea98}",
            removed: "\u{eadf}",
            fixed: "\u{ead8}",
            security: "\u{eb53}",
            tag: "\u{ea66}",
            calendar: "\u{eab0}",
        }
    }

    pub const fn emoji() -> Self {
        Icons {
            added: "➕",
            changed: "🔄",
            deprecated: "⚠️",
            removed: "🗑️",
            fixed: "🐛",
            security: "🛡️",
            tag: "🏷️",
            calendar: "🗓️",
        }
    }
}
