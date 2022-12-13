use crate::domain;
use crate::repository;
// use postgres::SimpleQueryMessage;

impl repository::Repository{
    pub fn consulta_id(&mut self, value: &str) -> Vec<domain::Product>{
        const SELECT_ID_START :  &str =  
            "SELECT * FROM reportes WHERE myid = $1::integer";
        
        let mut archivo: Vec<domain::Product> = Vec::new();
        let id: i32 = value.parse().unwrap();
        let result = self.db.query(SELECT_ID_START, &[&id]).unwrap();
        for row in result{
            let product = domain::Product{
                id: row.get(0),
                nombreReporte: row.get(1),
                informeproblema: row.get(2),
                informesolucion: row.get(3), 
                revisado: row.get(4),
                corregido: row.get(5),
                fecha: row.get(6),

            };
            archivo.push(product);
        }
        archivo
    }

    pub fn consulta_informe(&mut self, value: &str) -> Vec<domain::Product>{
        const SELECT_DESCRIPCION_START :  &str =  
            "SELECT * FROM reportes WHERE informeProblema ilike $1::varchar ORDER BY informeProblema";
            let mut archivo: Vec<domain::Product> = Vec::new();
            
            let value = format!("{}%",value);
            let result = self.db.query(SELECT_DESCRIPCION_START, &[&value]).unwrap();
            for row in result{
                let product = domain::Product{
                    id: row.get(0),
                    nombreReporte: row.get(1),
                    informeproblema: row.get(2),
                    informesolucion: row.get(3), 
                    revisado: row.get(4),
                    corregido: row.get(5),
                    fecha: row.get(6),
    
                };
                archivo.push(product);
            }
            archivo
    }

    pub fn consulta_nombre(&mut self, value: &str) -> Vec<domain::Product>{
        const SELECT_NOMBRE_START :  &str =  
            "SELECT * FROM reportes WHERE nombreReporte ilike $1::varchar ORDER BY nombreReporte";
            let mut archivo: Vec<domain::Product> = Vec::new();
            let value = format!("{}%",value);
            let result = self.db.query(SELECT_NOMBRE_START, &[&value]).unwrap();
        for row in result{
            let product = domain::Product{
                id: row.get(0),
                nombreReporte: row.get(1),
                informeproblema: row.get(2),
                informesolucion: row.get(3), 
                revisado: row.get(4),
                corregido: row.get(5),
                fecha: row.get(6),

            };
            archivo.push(product);
        }
        archivo
    }
}