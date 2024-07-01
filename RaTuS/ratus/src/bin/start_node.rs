use clap::Parser;
use ratus::start_example_raft_node;
use system::{ Logger, set_process_name };

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse the parameters passed by arguments.
    let options = Opt::parse();

    set_process_name(format!("ratus-{}", options.id).as_str());

    Logger::info("Starting RaTuS node", true);
    start_example_raft_node(options.id, options.http_addr).await
}
