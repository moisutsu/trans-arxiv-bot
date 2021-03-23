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
    #[clap(
        short,
        long,
        default_value = "15",
        about = "Interval to check the latest papers and tweet translated summary. (Unit: minute)"
    )]
    pub update_frequency: u64,
}
