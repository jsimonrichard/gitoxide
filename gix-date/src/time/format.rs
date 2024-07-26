use time::macros::format_description;

use crate::{
    time::{CustomFormat, Format},
    Time,
};

/// E.g. `2018-12-24`
pub const SHORT: CustomFormat = CustomFormat(format_description!("[year]-[month]-[day]"));

/// E.g. `Thu, 18 Aug 2022 12:45:06 +0800`
pub const RFC2822: CustomFormat = CustomFormat(format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
));

/// E.g. `Thu, 8 Aug 2022 12:45:06 +0800`. This is output by `git log --pretty=%aD`.
pub const GIT_RFC2822: CustomFormat = CustomFormat(format_description!(
    "[weekday repr:short], \
     [day padding:none] \
     [month repr:short] \
     [year] \
     [hour]:[minute]:[second] \
     [offset_hour sign:mandatory][offset_minute]"
));

/// E.g. `2022-08-17 22:04:58 +0200`
pub const ISO8601: CustomFormat = CustomFormat(format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
));

/// E.g. `2022-08-17T21:43:13+08:00`
pub const ISO8601_STRICT: CustomFormat = CustomFormat(format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
));

/// E.g. `123456789`
pub const UNIX: Format = Format::Unix;

/// E.g. `1660874655 +0800`
pub const RAW: Format = Format::Raw;

/// E.g. `Thu Sep 04 2022 10:45:06 -0400`, like the git `DEFAULT`, but with the year and time fields swapped.
pub const GITOXIDE: CustomFormat = CustomFormat(format_description!(
    "[weekday repr:short] [month repr:short] [day] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
));

/// E.g. `Thu Sep 4 10:45:06 2022 -0400`. This is output by `git log --pretty=%ad`.
pub const DEFAULT: CustomFormat = CustomFormat(format_description!(
    "[weekday repr:short] \
     [month repr:short] \
     [day padding:none] \
     [hour]:[minute]:[second] \
     [year] \
     [offset_hour sign:mandatory][offset_minute]"
));

/// Formatting
impl Time {
    /// Format this instance according to the given `format`.
    ///
    /// Use [`Format::Unix`], [`Format::Raw`] or one of the custom formats
    /// defined in the [`format`](mod@crate::time::format) submodule.
    pub fn format(&self, format: impl Into<Format>) -> String {
        self.format_inner(format.into())
    }

    fn format_inner(&self, format: Format) -> String {
        match format {
            Format::Custom(CustomFormat(format)) => self
                .to_time()
                .format(format)
                .expect("well-known format into memory never fails"),
            Format::Unix => self.seconds.to_string(),
            Format::Raw => self.to_bstring().to_string(),
        }
    }
}

impl Time {
    fn to_time(self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.seconds)
            .expect("always valid unix time")
            .to_offset(time::UtcOffset::from_whole_seconds(self.offset).expect("valid offset"))
    }
}
