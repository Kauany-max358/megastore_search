use megastore_search::{SistemaBusca, Produto};

#[test]
fn test_busca_produto_existente() {
    let mut sistema = SistemaBusca::new();
    let p = Produto { id: 10, nome: "Notebook".into(), marca: "Mega".into(), categoria: "TI".into() };
    sistema.indexar(p);
    
    assert!(sistema.buscar("notebook").is_some());
}