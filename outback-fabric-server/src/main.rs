use outback_branding::banner;
use outback_loader::{OutbackLoader, ServerPlatform};


fn main() {

    banner(
        "Outback-Fabric-01",
        "Fabric"
    );


    let loader = OutbackLoader::new(
        ServerPlatform::Fabric
    );


    loader.load();


    println!("Outback Fabric Server Started");

}