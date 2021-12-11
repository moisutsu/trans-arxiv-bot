use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opts {
    #[structopt(short, long, default_value = "cs.CL")]
    pub category: String,
    #[structopt(short, long, default_value = "en")]
    pub source_lang: String,
    #[structopt(short, long, default_value = "ja")]
    pub target_lang: String,
    #[structopt(
        short,
        long,
        default_value = "15",
        about = "Interval to check the latest papers and tweet translated summary. (Unit: minute)"
    )]
    pub update_frequency: u64,
}
