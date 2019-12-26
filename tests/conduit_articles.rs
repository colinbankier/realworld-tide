mod helpers;

use helpers::generate::new_user;
use helpers::test_db::get_test_repo;
use helpers::{create_articles, create_users};

use realworld_tide::conduit::articles;
use realworld_tide::conduit::users;
use realworld_tide::db::models::NewArticle;

#[test]
fn list_articles() {
    let repo = get_test_repo();

    let users = create_users(&repo, 5);
    let _articles = create_articles(&repo, users);
    let results = articles::find(&repo, Default::default()).expect("Failed to get articles");

    assert_eq!(results.len(), 5);
}

#[test]
fn insert_and_retrieve_article() {
    let repo = get_test_repo();
    let slug = "my_slug".to_string();

    let user = new_user();
    let user = users::insert(&repo, user).unwrap();

    let article = NewArticle {
        title: "My article".into(),
        slug: slug.clone(),
        description: "My article description".into(),
        body: "ohoh".into(),
        user_id: user.id,
    };
    let expected_article = articles::insert(&repo, article).unwrap();

    let (retrieved_article, retrieved_user) = articles::find_one(&repo, &slug).unwrap();
    assert_eq!(expected_article, retrieved_article);
    assert_eq!(user, retrieved_user);
}
