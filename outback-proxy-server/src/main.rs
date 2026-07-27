use outback_branding::banner;
use outback_loader::{OutbackLoader, ServerPlatform};


fn main() {

    banner(
        "Outback-Proxy-01",
        "Proxy"
    );


    let loader = OutbackLoader::new(
        ServerPlatform::Proxy
    );


    loader.load();


    println!("Outback Proxy Server Started");


    println!("Registered Servers:");

    println!("- Lobby-01");
    println!("- Survival-01");
    println!("- Modded-Forge-01");
    println!("- Modded-Fabric-01");


}