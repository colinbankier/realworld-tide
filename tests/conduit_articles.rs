mod helpers;

use helpers::test_db::get_test_repo;
use helpers::{create_article, create_articles, create_user, create_users};

use fake::fake;
use realworld_tide::conduit::articles;
use realworld_tide::db::models::{NewArticle, UpdateArticle};
use std::collections::HashSet;

#[test]
fn list_articles() {
    let repo = get_test_repo();

    let users = create_users(&repo, 5);
    let _articles = create_articles(&repo, users);
    let results = articles::find(&repo, Default::default()).expect("Failed to get articles");

    assert_eq!(results.len(), 5);
}

#[test]
fn delete_article() {
    let repo = get_test_repo();
    let n_articles = 5;
    let users = create_users(&repo, n_articles);
    let articles = create_articles(&repo, users);

    let slug = articles[0].slug.clone();
    articles::delete(&repo, &slug).expect("Failed to delete article");

    let results = articles::find(&repo, Default::default()).expect("Failed to get articles");
    assert_eq!(results.len() as i32, n_articles - 1);

    // The deleted article can't be fetched anymore
    let result = articles::find_one(&repo, &slug);
    assert!(result.is_err());
}

#[test]
fn update_article() {
    let repo = get_test_repo();

    let user = create_user(&repo);
    let article = create_article(&repo, &user);

    let update = UpdateArticle {
        title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
        description: Some(fake!(Lorem.paragraph(3, 10))),
        body: Some(fake!(Lorem.paragraph(10, 5))),
    };
    articles::update(&repo, update.clone(), article.slug.clone()).unwrap();

    let updated_article = articles::find_one(&repo, &article.slug).unwrap();
    assert_eq!(update.title, updated_article.content.title.into());
    assert_eq!(
        update.description,
        updated_article.content.description.into()
    );
    assert_eq!(update.body, updated_article.content.body.into());
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

#[test]
fn no_tags_if_there_are_no_articles() {
    let repo = get_test_repo();

    let result = articles::tags(&repo).unwrap();
    assert_eq!(0, result.len());
}

#[test]
fn no_tags_if_there_are_only_articles_without_tags() {
    let repo = get_test_repo();

    let user = create_user(&repo);

    let article = NewArticle {
        title: "My article".into(),
        slug: "my-slug".into(),
        description: "My article description".into(),
        body: "ohoh".into(),
        tag_list: vec![],
        user_id: user.id,
    };
    articles::insert(&repo, article).unwrap();

    let result = articles::tags(&repo).unwrap();
    assert!(result.is_empty());
}

#[test]
fn tags_works() {
    let repo = get_test_repo();

    let n_articles = 10;
    let users = create_users(&repo, n_articles);
    let articles = create_articles(&repo, users);

    let expected_tags: HashSet<String> = articles
        .into_iter()
        .map(|a| a.tag_list.into_iter())
        .flatten()
        .collect();

    let tags = articles::tags(&repo).unwrap();

    assert_eq!(expected_tags, tags);
}
