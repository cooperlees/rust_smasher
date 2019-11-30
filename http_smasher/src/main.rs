extern crate reqwest;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(
    name = "http_smasher",
    about = "Small tool to do a lot of http requests fast."
)]
struct Opt {
    #[structopt(short, long, default_value = "69")]
    requests: u32,
    url: String,
}

fn main() -> reqwest::Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let body = reqwest::get(&opt.url)?.text()?;
    println!("body = {:?}", body);
    Ok(())
}
