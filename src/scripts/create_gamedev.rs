use crate::storage::storage::Storage;

pub async fn execute(name: String) {
    let mongo_uri = "mongodb://localhost:27017".to_string();
    let storage = Storage::new(mongo_uri).await.unwrap();
    match storage.create_gamedev(name).await {
        Ok(new_gamedev) => {
            println!("Novo desenvolvedor de jogos inserido com sucesso:");
            println!("{:?}", new_gamedev);
        }
        Err(err) => {
            println!("Erro ao criar o novo desenvolvedor de jogos: {:?}", err);
        }
    };
}