use crate::domain;
use crate::repository;
use postgres::SimpleQueryMessage;

impl repository::Repository{
    pub fn all_products(&mut self) -> Vec<domain::Product> {
        const SELECT_ALL: &str = "SELECT id, name FROM products ORDER BY name";
        let mut products: Vec<domain::Product> = Vec::new();
        let result = self.db.simple_query(SELECT_ALL).unwrap();

        for data in result {
            if let SimpleQueryMessage::Row(row) = data {
                let product = domain::Product{
                    id: row.get(0).unwrap().parse().unwrap(),
                    name: row.get(1).unwrap().to_string(),
                };
                products.push(product);
            }


        }
    }
}