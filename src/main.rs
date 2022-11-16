use dockerClientUDP::Repository;

use std::{thread, time::Duration};
use std::net;
use std::io::*;
use std::net::UdpSocket;
use std::str;
use std::str::from_utf8;



//========Implementacion de socket UDP ==========//
//Espera como argumento la direccion del ip, para despues verificar si se hizo la conexion
pub fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let success = net::UdpSocket::bind(listen_on);
    let mut mySocketUDP;
    println!("{:?}",success);

    match success {
        //Mensaje de error en dado caso de no conectado
        Err(err) => panic!("No se pudo establecer la conexion: {}", err),

        Ok(sock) => {
            println!("Conectado a {}", listen_on);
            mySocketUDP = sock;
        }
    }
    mySocketUDP
}


fn leerMensaje(socket: net::UdpSocket) -> String {
    let mut buf = [0; 2048];
    println!("Reading data");
    let result = socket.recv_from(&mut buf);
    drop(socket);
    let mut req_msg ;
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
    msg
}
  
  
//Modificar para hacer una consulta en BD
pub fn enviarMensaje(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    println!("Mensaje enviado");
    let result = socket.send_to(&data, target);
    drop(socket);
    match result {
      Ok(amt) => println!("Se enviaron {} bytes", amt),
      Err(err) => panic!("Write error: {}", err)
    }
}

//Comunicacion entre el cliente y servidor
pub fn listen(listen_on: net::SocketAddr)-> String {
    let socket = socket(listen_on);
      let mut data = leerMensaje(socket);
      data
}





fn main(){
    println!("==========Inicio del servidor ============");

    //=======socket=====

    //Establecemos las direcciones ip
	let ip = net::Ipv4Addr::new(127, 0, 0, 1);
	let listen_addr = net::SocketAddrV4::new(ip, 1425);
	let send_addr = net::SocketAddrV4::new(ip, 1426);
	
    //==Seccion: Lo que le enviaremos al servidor==

    //Entrada de texto del println
	let mut entrada_consulta = String::new();
    println!("Consulta--Productos que empiezan con la letra: ");

    //Leer por consola
	let guardarEntradaTexto = std::io::stdin().read_line(&mut entrada_consulta);

    //Se guarda en un vector
	let data: Vec<u8> = entrada_consulta.as_bytes().to_vec();

	enviarMensaje(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), data);
	//println!("{:}", "=".repeat(80));


    //==Seccion: Respuesta del servidor==//
	let msg = listen(net::SocketAddr::V4(send_addr));
	println!("mensaje recibido: {:?}", msg);
	println!("{:}", "=".repeat(80));

    // Mostrar los productos que empiecen con m
    let mut repo = Repository::new();
    let products = repo.products_start_withid(&msg);
    print!("========== Productos que empiezan con M =========");
    for item in products{
        println!("| {} | {:<10} |", item.id, item.name);
    }
    //======fin del socket


    // let products = repo.all_products();
    // println!("========== ALL PRODUCTS =========");
    // for item in products{
    //     println!("| {} | {:<10} |", item.id, item.name);
    // }


    println!("Fin del programa");
}