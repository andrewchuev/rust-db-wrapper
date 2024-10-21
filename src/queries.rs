pub mod queries {
    use crate::models;
    use crate::models::post_model::Post;
    use crate::models::product_model::Product;
    use crate::Repository;

    pub async fn get_posts(repo: &Repository) {
        match repo.fetch_all::<Post>("wpbi_posts", Some(10), Some(0), Some("post_type='post'")).await {
            Ok(products) => {
                for product in products {
                    println!("{:?}", product);
                }
            }
            Err(e) => println!("Error fetching products: {}", e),
        }
    }

    pub async fn get_post(repo: &Repository, id: u32) {
        match repo.fetch_one::<Post>("wpbi_posts", id).await {
            Ok(post) => println!("Fetched product: {:?}", post),
            Err(e) => println!("Error fetching product by ID: {}", e),
        }
    }
}