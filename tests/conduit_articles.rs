mod helpers;

use helpers::test_server::get_repo;

use futures_executor::ThreadPool;
use realworld_tide::auth::encode_token;
use realworld_tide::conduit::articles;
use realworld_tide::conduit::users;
use realworld_tide::models::{NewArticle, NewUser};
use uuid::Uuid;

// use tokio_async_await_test::async_test;

// #[async_test]
// async fn test_list_articles() {
//     let repo = Repo::new();

//     let users =  create_users(&repo, 5).await ;
//     let _articles =  create_articles(&repo, users);
//     let results =
//         find(repo.clone(), Default::default()).await.expect("Failed to get articles");

//     assert_eq!(results.len(), 5);
// }

#[test]
fn insert_and_retrieve_article() {
    let runtime = ThreadPool::new().unwrap();
    runtime.spawn_ok(async move {
        let repo = get_repo();
        let slug = "my_slug".to_string();

        let user_id = Uuid::new_v4();
        let token = encode_token(user_id);
        let user = NewUser {
            username: "my_user".into(),
            email: "my_email@hotmail.com".into(),
            password: "somepass".into(),
            id: user_id,
            token,
        };
        let user = users::insert(&repo, user).unwrap();

        let article = NewArticle {
            title: "My article".into(),
            slug: slug.clone(),
            description: "My article description".into(),
            body: "ohoh".into(),
            user_id: user.id,
        };
        let expected_article = articles::insert(&repo, article).unwrap();

        let retrieved_article = articles::find_one(&repo, &slug).unwrap();
        assert_eq!(expected_article, retrieved_article);
    })
}
