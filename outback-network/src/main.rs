use std::collections::HashMap;


struct Server {

    name: String,

    server_type: String,

    online: bool,

}


struct NetworkManager {

    servers: HashMap<String, Server>,

}


impl NetworkManager {


    fn new() -> Self {

        Self {

            servers: HashMap::new(),

        }

    }


    fn register_server(
        &mut self,
        name: &str,
        server_type: &str
    ) {

        self.servers.insert(

            name.to_string(),

            Server {

                name: name.to_string(),

                server_type: server_type.to_string(),

                online: false,

            }

        );

    }


    fn list(&self) {


        println!("====== OUTBACK NETWORK ======");


        for server in self.servers.values() {


            println!(
                "{} | {} | {}",
                server.name,
                server.server_type,
                if server.online {
                    "ONLINE"
                } else {
                    "OFFLINE"
                }
            );


        }

    }


}



fn main() {


    println!("=================================");
    println!("       OUTBACK NETWORK");
    println!("=================================");


    let mut network = NetworkManager::new();


    network.register_server(
        "Lobby-01",
        "Paper"
    );


    network.register_server(
        "Survival-01",
        "Paper"
    );


    network.register_server(
        "Modded-Forge-01",
        "Forge"
    );


    network.register_server(
        "Modded-Fabric-01",
        "Fabric"
    );


    network.list();


}