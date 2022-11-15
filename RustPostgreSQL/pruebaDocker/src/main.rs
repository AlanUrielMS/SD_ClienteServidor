use pruebaDocker::Repository;

fn main(){
    println!("\n Inicio del programa");

    let mut repo = Repository::new();
    let products = repo.all_products();
    println!("========== ALL PRODUCTS =========");
    for item in products{
        println!("| {} | {:<10} |", item.id, item.name);
    }

    println!("Fin del programa");
}