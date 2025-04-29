use std::collections::HashMap;

fn main(){

    //criando o hashmap
    let mut estoque = HashMap::new();

    // 1. inserindo valores
    estoque.insert("banana", 100);
    estoque.insert("pepino", 50);
    estoque.insert("maçã", 25);
    estoque.insert("caqui", 12);

    // 2. acessar os valores
    if let Some(qtde) = estoque.get("maçã"){    
        println!("Temos {} maçãs no estoque", qtde);
    }   

    // 3. atualizado estoque com entry
    estoque.entry("pepino").and_modify (|qtde| *qtde += 100);
    if let Some(qtde) = estoque.get("pepino"){
        println!("Temos {:?} pepinos no estoque", qtde);
    }    

    // 4. Remoção com retorno
    estoque.remove("pepino");
    println!("Removido: {:?}", estoque); 
    
    // 5. Filtrar as frutas com qtde acima de 100
    estoque.retain(|fruta, &mut qtde| qtde>100);
    println!("{:?}", estoque);    

    // 6. Limpeza
    estoque.clear();
    println!("{:?}", estoque);    

}