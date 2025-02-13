
# SD_ClienteServidor
En este repositorio se trbajará la actividad de un Sistema Cliente servidor y una base de datos.
Se debe de seguir la siguinte serie de comandos para la correcta instalacion y desarrollo de la implementacion cliente servidor con docker y Rust.

## Comandos de instalación de Postgresql y Pgadmin
### intalacion Postgresql
- sudo apt update
- sudo apt install postgresql postgresql-contrib
- sudo su - postgres
- psql
- \l

### Creacion del usuario y asignacion de atributos de superusiario
- create user NombreDeTuUsuario with password 'NombreDeTuPassword';
- create database NombreDeTuBaseDeDatos with owner NombreDeTuUsuario;
- alter user NombreDeTuUsuario with superuser;

### instalacion de IDE pgAdmin4
- sudo apt install curl
- curl https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo apt-key add
- sudo sh -c 'echo "deb https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list && 
- apt update'
- sudo apt install pgadmin4

## Creacion de la BD
Una vez instalado aplicación pgAdmin 4, abrela e introduce tu contraseña antes creada, despues abrimos un Qery Tools y creamos un base de datos de ejemplo,la que se utilizo fue:

### Creacion de la tabla
```
CREATE TABLE reportes (
	myid INTEGER NOT NULL PRIMARY KEY,
	nombreReporte VARCHAR(150) NOT NULL,
	informeProblema varchar(500) NOT NULL,
	informeSolucion varchar(500) NOT NULL,
	revisado varchar(5) NOT NULL,
	corregido varchar(5) NOT NULL,
	fecha varchar(12) NOT NULL
);

INSERT INTO reportes(myid,nombreReporte,informeProblema,informeSolucion,revisado,corregido,fecha)
VALUES (1, 
		'Red001',
		'El usuairio presenta falla de red ....',
		'El Fallo se soluciono mediante.....',
		'false',
		'false',
		'10-11-2022');
		
INSERT INTO reportes(myid,nombreReporte,informeProblema,informeSolucion,revisado,corregido,fecha)
VALUES (2, 
		'Red002',
		'El usuairio presenta falla de red ....',
		'El Fallo se soluciono mediante.....',
		'true',
		'false',
		'10-11-2022');
		
INSERT INTO reportes(myid,nombreReporte,informeProblema,informeSolucion,revisado,corregido,fecha)
VALUES (3, 
		'System001',
		'El usuairio presenta falla de sistema ....',
		'El Fallo se soluciono mediante.....',
		'False',
		'false',
		'10-11-2022');
```

## Docker instalación 
Dado que se trabajará en algunas ocasiones remotamente, usaremos Docker, la cual facilita la migración entre diferentes sistemas y garantiza el funcionamiento íntegro, evitando cualquier tipo de posible problema de dependencia entre versiones de software de los diferentes hosts.

Para su instalación seguimos las instrucciones de la página oficial https://docs.docker.com/engine/install/ubuntu/


## Configuracion de Docker + postgresSQL
Para la configuración nos pedira estableceer una contraseña, asi como indicar el puerto donde se conectara el usuario postgres y establacer que base de datos usaremos, asi debemos de saber como cambiar el nombre de un contenedor, como detener un contenedor y como removerlo. Para un mejor entendimiento segui este tutorial https://www.youtube.com/watch?v=hVrKX2RtigQ&t=1047s

Esto resulta muy util, porque no debemos de establecer a cada rato la instancia de postgreSQL en Docker

## Código

Para poder ejecutar el codigo de deben descargar ambas carpetas tanto la del cliente como la del servidor, una vez realizado esto se debe abrir una terminal por cada una y correr el comando ***cargo run*** primero en el servidor y despues en el cliente.
