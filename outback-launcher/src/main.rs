use std::process::Command;


struct Server {

    name: &'static str,

    jar: &'static str,

}



fn start_server(server: &Server) {


    println!(
        "Starting {}",
        server.name
    );


    let result = Command::new("java")
        .args([
            "-jar",
            server.jar,
            "nogui"
        ])
        .spawn();


    match result {

        Ok(_) => {

            println!(
                "{} started!",
                server.name
            );

        }


        Err(error) => {

            println!(
                "Failed to start {}: {}",
                server.name,
                error
            );

        }

    }

}



fn main() {


    println!("=================================");
    println!("       OUTBACK LAUNCHER");
    println!("=================================");


    let lobby = Server {

        name: "Lobby-01",

        jar: "outback-paper.jar",

    };


    start_server(&lobby);


}