#![cfg(feature = "nightly")]

use rustfmt_nightly::{Config, Edition, EmitMode, Input, Session};

/// Programmatically runs rustfmt on a `String`.
pub fn rustfmt<S>(module: S) -> Result<String, failure::Error>
where
    S: Into<String>,
{
    let input = Input::Text(module.into());

    let mut config = Config::default();
    config.set().edition(Edition::Edition2018);
    config.set().emit_mode(EmitMode::Stdout);
    // config.set().max_width(200);

    let mut output = Vec::new();
    {
        let mut session = Session::new(config, Some(&mut output));
        let _format_report = session.format(input)?;
    }
    let s = String::from_utf8(output)?;
    Ok(s)
}
