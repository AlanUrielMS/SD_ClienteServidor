use postgres::{
    Client, NoTls, 
    SimpleQueryMessage::{CommandComplete,Row}
};
fn main() {
    println!("## Inicio conexion BD");

    const CONFIG: &str = "postgresql://alanbc:password@127.0.0.1:5432/myConsultas?application_name=pruebadocker=app";


    let client: Result<Client, postgres::Error> = Client::connect(CONFIG, NoTls);
    
    let mut client = match client {
        Err(connection_error) => {
            eprintln!("\n ## ERROR ## error al conectar la base de datos\n{}\n",connection_error );
            return;
        },
        Ok(client) => {
            println!("## info ## conectado al banco de datos");
            client
        }
    };

    const SELECT: &str = "SELECT VERSION()";
    let query_results = client.simple_query(SELECT);

    match query_results {
        Err(query_error) => println!("Error - no se pudo ejecutar la consulta\n{}\n",query_error),
        Ok(results) => {
            for result in results{
                match result {

                    
                    CommandComplete(num_rows) => println!("{} registros\n", num_rows),
                    Row(row) => {
                        let num_columns ) row.len();
                        // for (column!())
                    }
                    _ => println!("Todo bien :D"),
                    
                }
            }
        }
    }


    // if let Ok(results) = &query_result{
    //     println!("Ejecutado con exito");
    // }

    if let Err(error) = client.close(){
        eprintln!("## Error al cerrar la conexion \n{}\n",error );
    }

    else{
        println!("## Conexion cerrada");
    }
    println!("## fin de programa");

}
