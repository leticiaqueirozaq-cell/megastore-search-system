use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Product {
    id: u32,
    name: String,
    brand: String,
    category: String,
}

struct ProductIndex {
    name_index: HashMap<String, Vec<Product>>,
}

impl ProductIndex {

    fn new() -> Self {
        Self {
            name_index: HashMap::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.name_index
            .entry(product.name.clone())
            .or_insert(Vec::new())
            .push(product);
    }

    fn search(&self, name: &str) -> Option<&Vec<Product>> {
        self.name_index.get(name)
    }
}

fn main() {

    let mut index = ProductIndex::new();

    let product = Product {
        id: 1,
        name: "Notebook".to_string(),
        brand: "Dell".to_string(),
        category: "Eletronicos".to_string(),
    };

    index.add_product(product);

    if let Some(results) = index.search("Notebook") {
        println!("Produtos encontrados:");
        for p in results {
            println!("{:?}", p);
        }
    }
}
