use megastore_search::{SistemaBusca, Produto};

fn main() {
    let mut sistema = SistemaBusca::new();
    let p = Produto { 
        id: 1, 
        nome: "Smartphone".into(), 
        marca: "TechMega".into(), 
        categoria: "Eletrônicos".into() 
    };

    sistema.indexar(p);
    println!("--- MegaStore: Sistema de Busca Online ---");
    println!("Catálogo indexado com sucesso via Tabela Hash!");
}