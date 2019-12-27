mod helpers;

use helpers::generate::new_user;
use helpers::test_db::get_test_repo;
use helpers::{create_article, create_articles, create_user, create_users};

use fake::fake;
use realworld_tide::conduit::articles;
use realworld_tide::conduit::users;
use realworld_tide::db::models::{NewArticle, UpdateArticle};

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
        tag_list: vec!["a tag!".to_string()],
        user_id: user.id,
    };
    let expected_article = articles::insert(&repo, article).unwrap();

    let (retrieved_article, retrieved_user, _) = articles::find_one(&repo, &slug).unwrap();
    assert_eq!(expected_article, retrieved_article);
    assert_eq!(user, retrieved_user);
}

#[test]
fn update_article() {
    let repo = get_test_repo();

    let user = create_user(&repo);
    let article = create_article(&repo, user.clone());

    let update = UpdateArticle {
        title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
        description: Some(fake!(Lorem.paragraph(3, 10))),
        body: Some(fake!(Lorem.paragraph(10, 5))),
    };
    articles::update(&repo, update.clone(), article.slug.clone()).unwrap();

    let (updated_article, _, _) = articles::find_one(&repo, &article.slug).unwrap();
    assert_eq!(update.title, updated_article.title.into());
    assert_eq!(update.description, updated_article.description.into());
    assert_eq!(update.body, updated_article.body.into());
}

#[test]
fn you_cannot_update_an_article_that_does_not_exist() {
    let repo = get_test_repo();
    let slug = "A random slug";

    let update = UpdateArticle {
        title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
        description: Some(fake!(Lorem.paragraph(3, 10))),
        body: Some(fake!(Lorem.paragraph(10, 5))),
    };
    let result = articles::update(&repo, update.clone(), slug.to_string());
    assert!(result.is_err());
}
