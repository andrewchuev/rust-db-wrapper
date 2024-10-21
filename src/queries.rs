pub mod queries {
    use crate::models;
    use crate::models::post_model::Post;
    use crate::Repository;

    pub async fn get_posts(repo: Repository) {
        match repo.fetch_all::<Post>("wpbi_posts", Some(100), Some(0), Some("post_type='post'")).await {
            Ok(products) => {
                for product in products {
                    println!("{:?}", product);
                }
            }
            Err(e) => println!("Error fetching products: {}", e),
        }
    }
}