use clap::{crate_authors, crate_description, crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    #[clap(short, long, default_value = "cs.CL")]
    pub category: String,
    #[clap(short, long, default_value = "en")]
    pub source_lang: String,
    #[clap(short, long, default_value = "ja")]
    pub target_lang: String,
    #[clap(long, default_value = "0")]
    pub range_days: i64,
    #[clap(long, default_value = "1")]
    pub range_hours: i64,
}
