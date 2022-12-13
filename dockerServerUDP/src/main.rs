use dockerServerUDP::Repository;

use std::any::Any;
use std::{thread, time::Duration};
use std::net;
use std::net::UdpSocket;
use std::str;
use serde::{Serialize, Deserialize};
use bincode::{self, deserialize};


#[derive(Serialize, Deserialize, Debug, Default)]
//Estructura generica mutable de tipo solicitud
struct StructSolicitud{
    tipo_solicitud: String, //Tipo de consulta
    dato_solicitud: String,  //
    tamanio: i32,
}

fn consulta_nueva(tipo_solicitud: String, dato_solicitud: String, tamanio: i32) -> StructSolicitud{
    StructSolicitud {
        tipo_solicitud: tipo_solicitud,
        dato_solicitud: dato_solicitud,
        tamanio: tamanio,
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct File{
    id: i32,
    nombre: String,
    descripcion: String,
    ruta: String,
    tipo: String,
    tamanio : i32,
    revisado: String,
    corregido: String,
    fecha: String,
}
impl File {
    fn respuesta_consulta( id: i32, nombre: String, descripcion: String, ruta: String, tipo: String, tamanio : i32, revisado: String, corregido: String, fecha: String) -> File {
        File{
            id: id,
            nombre: nombre,
            descripcion: descripcion,
            ruta: ruta,
            tipo: tipo,
            tamanio : tamanio,
            revisado: revisado,
            corregido: corregido,
            fecha: fecha,
        }   
    }    
}



pub fn enviarMensaje(send_addr: net::SocketAddr, target: net::SocketAddr, consulta: String) {
    
    let socket = UdpSocket::bind(send_addr).expect("erroorrrr");
    
    let data: Vec<u8> = consulta.as_bytes().to_vec();
    let result = socket.send_to(&data, target);
    println!("Mensaje enviado");

    drop(socket);
    match result {
      Ok(amt) => println!("Se enviaron {} bytes", amt),
      Err(err) => panic!("Write error: {}", err)
    }
}

fn consulta_id(listen_on: net::SocketAddr, send_addr: net::SocketAddr){
    let socket = UdpSocket::bind(listen_on).expect("erroorrrr");
    //socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    let mut buf = [0; 10];
    let mut req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let mut msg;
    drop(socket);
    match result {
    Ok((amt, src)) => {
    let buf = &mut buf[..amt];
    println!("Received data from {}", src);
    req_msg = str::from_utf8(&buf).unwrap();
    msg = String::from(req_msg);
    },
    Err(err) => panic!("Read error: {}", err)
    }
    println!("mensaje recibido del cliente: {:?}", msg);
    println!("{:}", "=".repeat(80));

    let mut repo = Repository::new();
    let file = repo.consulta_id(&msg);


    print!("==========");
    let mut consulta = String::new();
    
    for item in file{
         //println!("| {} | {} |{} |{} |{} |{} |{} |", item.id, item.nombreReporte,item.informeproblema,item.informesolucion,item.revisado,item.corregido,item.fecha);
         consulta = item.id.to_string();
         consulta.push_str("| ");
         consulta.push_str(&item.nombre);
         consulta.push_str("| ");
         consulta.push_str(&item.descripcion);
         consulta.push_str("| ");
         consulta.push_str(&item.ruta);
         consulta.push_str("| ");
         consulta.push_str(&item.tipo);
         consulta.push_str("| ");
         consulta.push_str(&item.tamanio.to_string());
         consulta.push_str("| ");
         consulta.push_str(&item.revisado);
         consulta.push_str("| ");
         consulta.push_str(&item.corregido);
         consulta.push_str("| ");
         consulta.push_str(&item.fecha);
         consulta.push_str("|");
    }
    println!("{:?}",consulta);
    
    enviarMensaje(listen_on, send_addr, consulta)

}

fn consulta_informe(listen_on: net::SocketAddr, send_addr: net::SocketAddr) {
    let socket = UdpSocket::bind(listen_on).expect("erroorrrr");
    //socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    let mut buf = [0; 10];
    let mut req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let mut msg;
    match result {
    Ok((amt, src)) => {
    let buf = &mut buf[..amt];
    println!("Received data from {}", src);
    req_msg = str::from_utf8(&buf).unwrap();

    msg = String::from(req_msg);
    },
    Err(err) => panic!("Read error: {}", err)
    }
    println!("mensaje recibido del cliente: {:?}", msg);
    println!("{:}", "=".repeat(80));

    let mut repo = Repository::new();
    let file = repo.consulta_informe(&msg);
    print!("==========");
    let mut consulta = String::new();
    for item in file{
        //println!("| {} | {} |{} |{} |{} |{} |{} |", item.id, item.nombreReporte,item.informeproblema,item.informesolucion,item.revisado,item.corregido,item.fecha);
        consulta = item.id.to_string();
        consulta.push_str("| ");
        consulta.push_str(&item.nombre);
        consulta.push_str("| ");
        consulta.push_str(&item.descripcion);
        consulta.push_str("| ");
        consulta.push_str(&item.ruta);
        consulta.push_str("| ");
        consulta.push_str(&item.tipo);
        consulta.push_str("| ");
        consulta.push_str(&item.tamanio.to_string());
        consulta.push_str("| ");
        consulta.push_str(&item.revisado);
        consulta.push_str("| ");
        consulta.push_str(&item.corregido);
        consulta.push_str("| ");
        consulta.push_str(&item.fecha);
        consulta.push_str("|");
   }
   println!("{:?}",consulta);
   
   enviarMensaje(listen_on, send_addr, consulta)
}

fn consulta_nombre(listen_on: net::SocketAddr, send_addr: net::SocketAddr) {
    let socket = UdpSocket::bind(listen_on).expect("erroorrrr");
    //socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    let mut buf = [0; 10];
    let mut req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let mut msg;
    match result {
    Ok((amt, src)) => {
    let buf = &mut buf[..amt];
    println!("Received data from {}", src);
    req_msg = str::from_utf8(&buf).unwrap();

    msg = String::from(req_msg);
    },
    Err(err) => panic!("Read error: {}", err)
    }
    println!("mensaje recibido del cliente: {:?}", msg);
    println!("{:}", "=".repeat(80));

    let mut repo = Repository::new();
    let file = repo.consulta_nombre(&msg);
    print!("==========");
    let mut consulta = String::new();
    for item in file{
        //println!("| {} | {} |{} |{} |{} |{} |{} |", item.id, item.nombreReporte,item.informeproblema,item.informesolucion,item.revisado,item.corregido,item.fecha);
        consulta = item.id.to_string();
        consulta.push_str("| ");
        consulta.push_str(&item.nombre);
        consulta.push_str("| ");
        consulta.push_str(&item.descripcion);
        consulta.push_str("| ");
        consulta.push_str(&item.ruta);
        consulta.push_str("| ");
        consulta.push_str(&item.tipo);
        consulta.push_str("| ");
        consulta.push_str(&item.tamanio.to_string());
        consulta.push_str("| ");
        consulta.push_str(&item.revisado);
        consulta.push_str("| ");
        consulta.push_str(&item.corregido);
        consulta.push_str("| ");
        consulta.push_str(&item.fecha);
        consulta.push_str("|");
   }
   println!("{:?}",consulta);
   
   enviarMensaje(listen_on, send_addr, consulta)
}

// fn leerMensaje(socket: net::UdpSocket) -> String {
//     let mut buf = [0; 2048];
//     println!("Reading data");
//     let result = socket.recv_from(&mut buf);
//     drop(socket);
//     let mut req_msg ;
//     let mut msg;
//     match result {
//       Ok((amt, src)) => {
//         let buf = &mut buf[..amt];
//         println!("Received data from {}", src);
//         req_msg = str::from_utf8(&buf).unwrap();
//         msg = String::from(req_msg);
//       },
//       Err(err) => panic!("Read error: {}", err)
//     }
//     msg
// }
  
  
//Comunicacion entre el cliente y servidor
pub fn listen(listen_on: net::SocketAddr, send_addr: net::SocketAddr) {
    let socket = UdpSocket::bind(listen_on).expect("erroorrrr");
    //socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    let mut buffer = [0; 2048];
    //let mut buffer: Vec<u8>;
        //let mut req_msg ;
    let result = socket.recv_from(&mut buffer);
    println!("el ssid es: {:?}",buffer);
    //println!("resultado: {:?}",result);

   drop(socket);

    //let mut mensaje_recibido:File;
    let mensaje_recibido:File;

    match result {
        Ok((amt, src)) => {

            let buffer = &mut buffer[..amt];
            println!("Received data from {}", src);
            println!("el ssid es: {:?}",buffer);
            //req_msg = str::from_utf8(&buf).unwrap();
            
            //FIXME: Deserializar el mensaje
            // let mut cursor = &buf[..];
            // let mut msg:File = bincode::deserialize_from(&cursor)?;
            // // println!("mensaje deserializado {:?}",msg);
            // msg.trailing = cursor.to_owned();
            // println!("{:#?}", msg);
            
            mensaje_recibido = bincode::deserialize(&buffer).unwrap();
            
        },
        Err(err) => panic!("Error al leer los datos: {}", err)
    }
    // println!("mensaje recibido del cliente: {:?}", msg);
    // println!("{:}", "=".repeat(80));


	//Tipos de consultas    
    if mensaje_recibido.id == 1 {
        consulta_id(listen_on,send_addr);
    }else if mensaje_recibido.id == 2 {
        consulta_informe(listen_on,send_addr);
    }else if mensaje_recibido.id == 3 {
        consulta_nombre(listen_on,send_addr);
    }

}


fn main(){
    println!("\n Inicio del programa");

    //=======socket=====
	/*let ip_server = net::Ipv4Addr::new(172, 30, 5, 27);
    let ip_client = net::Ipv4Addr::new(172, 30, 5, 2);
	let my_dir = net::SocketAddrV4::new(ip_server, 1425);
	let send_dir = net::SocketAddrV4::new(ip_client, 1426);
	println!("Mi direccion {} ",my_dir);*/
	
    let ip_server = net::Ipv4Addr::new(127, 0, 0, 1);
    let ip_client = net::Ipv4Addr::new(127, 0, 0, 1);
	let my_dir = net::SocketAddrV4::new(ip_server, 1425);
	let send_dir = net::SocketAddrV4::new(ip_client, 1426);
	println!("Mi direccion {} ",my_dir);

	println!("{:}", "=".repeat(80));
    loop {
        //==== Seccion: peticion del cliente ====
	    listen(net::SocketAddr::V4(my_dir),net::SocketAddr::V4(send_dir));
        
        //==== Seccion: El servidor le devuelve la respuesta al cliente ====
	    thread::sleep(Duration::from_millis(1000));        
    }

	
    println!("Fin del programa");
}
