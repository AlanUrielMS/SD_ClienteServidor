
use std::net;
use std::str;
use std::str::FromStr;
use dockerClientUDP::domain;
use dockerClientUDP::domain::Solicitud;
use serde::{Serialize, Deserialize};
use bincode;

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

//Estructura para que pueda interpretar el cliente lo que le envio al servidor






//========Implementacion de socket UDP ==========//
//Espera como argumento la direccion del ip, para despues verificar si se hizo la conexion
pub fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let success = net::UdpSocket::bind(listen_on);
    let mut mySocketUDP;
    //println!("{:?}",success);

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


fn leer_mensaje(socket: net::UdpSocket) -> String {
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
pub fn enviar_mensaje(send_addr: net::SocketAddr, target: net::SocketAddr, data: Vec<u8>) {
    let socket = net::UdpSocket::bind(send_addr).expect("couldn't bind to address");
    //socket.connect(target).expect("connect function failed");
    println!("Se realizo conexion del socket");

    println!("el ssid es: {:?}",data);
    let envioMensaje = socket.send_to(&data, target);
    //El uso principal del rasgo Drop es liberar los recursos que posee la instancia del implementador.    
    drop(socket);
    
    match envioMensaje {
      Ok(amt) => println!("Se enviaron {} bytes", amt),
      Err(err) => panic!("Write error: {}", err)
    }   
}

//Comunicacion entre el cliente y servidor
pub fn listen(listen_on: net::SocketAddr)-> String {
    let socket = socket(listen_on);
      let mut data = leer_mensaje(socket);
      data
}


fn main(){
    
    println!("========= Inicio del Cliente ============");

    //=======socket=====
    //Conexion LAN//let socket = UdpSocket::bind("172.30.5.27:34255").expect("erroorrrr");
    //Conexion LAN//socket.connect("172.30.5.2:34254").expect("Error al establecer una conexion");

    //Establecemos las direcciones ip
    /*let ip_server = net::Ipv4Addr::new(172, 30, 5, 27);
    let ip_client = net::Ipv4Addr::new(172, 30, 5, 2);
    let listen_addr = net::SocketAddrV4::new(ip_server, 34255);
    let send_addr = net::SocketAddrV4::new(ip_client, 34254);*/
	
	let ip_server = net::Ipv4Addr::new(127, 0, 0, 1);
    let ip_client = net::Ipv4Addr::new(127, 0, 0, 1);
	let listen_addr = net::SocketAddrV4::new(ip_server, 1425);
	let send_addr = net::SocketAddrV4::new(ip_client, 1426);
	println!("Mi direccion {} ",send_addr);

	
    println!("DESPLEGANDO MENU");
    println!("1) Consulta de id ");
    println!("2) Consulta por informe problema ");
    println!("3) Consulta por nombre reporte");
    println!("Que tipo de consulta desea realizar?: ");
    let mut entrada_seleccion = String::new();
    std::io::stdin().read_line(&mut entrada_seleccion).ok().expect("Error al leer de teclado");
    let seleccion: i32 = i32::from_str(&entrada_seleccion.trim()).unwrap();
    let entrada_seleccion = &entrada_seleccion[0..1];

    // StructSolicitud::tipoSolicitud = String::from(entrada_seleccion);
    
    
    //let seleccion_b: Vec<u8> = entrada_seleccion.as_bytes().to_vec();
    //FIXME: enviarMensaje(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), seleccion_b);


    if seleccion == 1 {

        //========= Seccion: Lo que le enviaremos al servidor =========
        println!("======= Consulta por ID =======");
        //Entrada de texto del println
	    println!("Ingrese el id a buscar: ");
        let mut entrada_id = String::new();
        let guardarEntradaTexto = std::io::stdin().read_line(&mut entrada_id);
        //Se acorta el string para evitar la lectura del salto de linea
        let entrada_id = &entrada_id[0..1];
    
        
        // let entradaDatoPDU = structSolicitud.datoSolicitud;

        //FIXME: Se guarda en un vector
        // let data: Vec<u8>;
        // let mut udpVector: Vec<Solicitud> = Vec::new();
        // udpVector.push(structSolicitud);


        //mensaje completo
        let ssid = consulta_nueva(entrada_seleccion.to_string(), entrada_id.to_string(), 1);
        println!("el ssid es: {:?}",ssid);
        //FIXME:Serializar
        let mensaje_serializado = bincode::serialize(&ssid).unwrap();
                        
	    enviar_mensaje(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), mensaje_serializado);

        //====== Seccion: Respuesta del servidor =============//
	    let msg = listen(net::SocketAddr::V4(send_addr));
	    println!("mensaje recibido de cliente");
        println!("{}",msg);

    }else if seleccion == 2{
        println!("Consulta de informe del problema, ingrese un carater(r=red,s=sistema)");
        //Entrada de texto del println
	    let mut entrada_consulta = String::new();
        println!("Ingrese el tipo de reporte a buscar: ");
        //Leer por consola
	    let guardarEntradaTexto = std::io::stdin().read_line(&mut entrada_consulta);
        //Se acorta el string para evitar la lectuira del salto de linea
        let entrada_consulta = &entrada_consulta[0..1];
        //Se guarda en un vector
	    let data: Vec<u8> = entrada_consulta.as_bytes().to_vec();
	    enviar_mensaje(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), data);
        //==Seccion: Respuesta del servidor==//
	    let msg = listen(net::SocketAddr::V4(send_addr));
      println!("mensaje recibido de cliente");
      println!("{}",msg);

    }else if seleccion == 3{
        println!("Consulta reporte  por nombre: ");
        //Entrada de texto del println
	    let mut entrada_consulta = String::new();
        //Leer por consola
	    let _guardar_entrada_texto = std::io::stdin().read_line(&mut entrada_consulta);
        //Se acorta el string para evitar la lectuira del salto de linea
        let entrada_consulta = &entrada_consulta[0..1];
        //Se guarda en un vector
	    let data: Vec<u8> = entrada_consulta.as_bytes().to_vec();
	    enviar_mensaje(net::SocketAddr::V4(send_addr), net::SocketAddr::V4(listen_addr), data);
        
        //==Seccion: Respuesta del servidor==//
	    let msg = listen(net::SocketAddr::V4(send_addr));
	    println!("mensaje recibido de cliente");
      println!("{}",msg);
    }

    
    println!("Fin del programa");
}
