mod error;

use std::time::Duration;

use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub use self::error::{Error, Result};

// ——> Time
// =====================
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339)
        .expect("OffsetDateTime should be formattable to Rfc3339") //  TODO: need to check if safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::from_secs_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::DateFailParse(moment.into()))
}

// ——> Base64
// =====================
pub fn b64u_encode(content: &str) -> String {
    base64_url::encode(content)
}
pub fn b64u_decode(content: &str) -> Result<String> {
    let decoded = base64_url::decode(content)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB64uDecode)?;

    Ok(decoded)
}
