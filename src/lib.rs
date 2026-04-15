use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

pub struct SistemaBusca {
    pub indice: HashMap<String, Produto>,
}

impl SistemaBusca {
    pub fn new() -> Self {
        Self { indice: HashMap::new() }
    }

    pub fn indexar(&mut self, p: Produto) {
        self.indice.insert(p.nome.to_lowercase(), p);
    }

    pub fn buscar(&self, nome: &str) -> Option<&Produto> {
        self.indice.get(&nome.to_lowercase())
    }
}