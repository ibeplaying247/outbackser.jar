pub enum ServerPlatform {

    Paper,

    Forge,

    Fabric,

    Proxy,

}


pub struct OutbackLoader {

    pub platform: ServerPlatform,

}


impl OutbackLoader {


    pub fn new(platform: ServerPlatform) -> Self {

        Self {
            platform
        }

    }


    pub fn load(&self) {


        match self.platform {


            ServerPlatform::Paper => {

                println!("Outback Loader");
                println!("Platform: Paper");
                println!("Loading plugins...");

            }


            ServerPlatform::Forge => {

                println!("Outback Loader");
                println!("Platform: Forge");
                println!("Loading mods...");

            }


            ServerPlatform::Fabric => {

                println!("Outback Loader");
                println!("Platform: Fabric");
                println!("Loading mods...");

            }


            ServerPlatform::Proxy => {

                println!("Outback Loader");
                println!("Platform: Proxy");
                println!("Loading proxy plugins...");

            }

        }

    }

}