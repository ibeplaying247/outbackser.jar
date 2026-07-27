use outback_branding::banner;
use outback_loader::{OutbackLoader, ServerPlatform};


fn main() {

    banner(
        "Outback-Hub-01",
        "Paper"
    );


    let loader = OutbackLoader::new(
        ServerPlatform::Paper
    );


    loader.load();


    println!("Outback Paper Server Started");

}