use dockerServerUDP::Repository;

use std::{thread, time::Duration};
use std::net;
use std::net::UdpSocket;
use std::str;


/*use serde::{Serialize, Deserialize};
use bincode;*/

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
    let req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let msg;
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
         consulta.push_str(&item.id.to_string());
         consulta.push_str("| ");
         consulta.push_str(&item.nombreReporte);
         consulta.push_str("| ");
         consulta.push_str(&item.informeproblema);
         consulta.push_str("| ");
         consulta.push_str(&item.informesolucion);
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
    let req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let msg;
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
         consulta.push_str(&item.id.to_string());
         consulta.push_str("| ");
         consulta.push_str(&item.nombreReporte);
         consulta.push_str("| ");
         consulta.push_str(&item.informeproblema);
         consulta.push_str("| ");
         consulta.push_str(&item.informesolucion);
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
    let req_msg ;
    let result = socket.recv_from(&mut buf);
    
    let msg;
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
         consulta.push_str(&item.id.to_string());
         consulta.push_str("| ");
         consulta.push_str(&item.nombreReporte);
         consulta.push_str("| ");
         consulta.push_str(&item.informeproblema);
         consulta.push_str("| ");
         consulta.push_str(&item.informesolucion);
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

fn leerMensaje(socket: net::UdpSocket) -> String {
    let mut buf = [0; 2048];
    println!("Reading data");
    let result = socket.recv_from(&mut buf);
    drop(socket);
    let req_msg ;
    let msg;
    match result {
      Ok((amt, src)) => {
          let buf = &mut buf[..amt];
        println!("Received data from {}", src);
        req_msg = str::from_utf8(&buf).unwrap();
        msg = String::from(req_msg);
      },
      Err(err) => panic!("Read error: {}", err)
    }
    msg
}
  
  
//Modificar para hacer una consulta en BD


//Comunicacion entre el cliente y servidor
pub fn listen(listen_on: net::SocketAddr, send_addr: net::SocketAddr) {
    let socket = UdpSocket::bind(listen_on).expect("erroorrrr");
    //socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    let mut buf = [0; 10];
    let req_msg ;
    let result = socket.recv_from(&mut buf);
    
    drop(socket);
    let msg;
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

	    
    if msg == "1" {
        consulta_id(listen_on,send_addr);
    }else if(msg == "2"){
        consulta_informe(listen_on,send_addr);
    }else if(msg == "3"){
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

    //Seccion: peticion del cliente
    //Hacer que el servidor le responda con un string
    //let msg = String::new();
	listen(net::SocketAddr::V4(my_dir),net::SocketAddr::V4(send_dir));
	
    //Seccion: El servidor le devuelve la respuesta al cliente
	thread::sleep(Duration::from_millis(1000));
	
    println!("Fin del programa");
}