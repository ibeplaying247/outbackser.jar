use outback_branding::banner;
use outback_loader::{OutbackLoader, ServerPlatform};


fn main() {

    banner(
        "Outback-Modded-01",
        "Forge"
    );


    let loader = OutbackLoader::new(
        ServerPlatform::Forge
    );


    loader.load();


    println!("Outback Forge Server Started");

}
