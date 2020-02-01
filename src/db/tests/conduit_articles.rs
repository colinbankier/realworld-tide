mod helpers;

use helpers::test_db::get_test_repo;
use helpers::{create_articles, create_user, create_users};

use realworld_db::models::{NewArticle, User};
use realworld_db::queries::articles;
use std::collections::HashSet;

#[test]
fn list_articles() {
    let repo = get_test_repo();

    let users: Vec<User> = create_users(&repo, 5).into_iter().map(|(u, _)| u).collect();
    let _articles = create_articles(&repo, users);
    let results = articles::find(&repo, Default::default()).expect("Failed to get articles");

    assert_eq!(results.len(), 5);
}

#[test]
fn delete_article() {
    let repo = get_test_repo();
    let n_articles = 5;
    let users: Vec<User> = create_users(&repo, n_articles)
        .into_iter()
        .map(|(u, _)| u)
        .collect();
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
fn no_tags_if_there_are_no_articles() {
    let repo = get_test_repo();

    let result = articles::tags(&repo).unwrap();
    assert_eq!(0, result.len());
}

#[test]
fn no_tags_if_there_are_only_articles_without_tags() {
    let repo = get_test_repo();

    let user = create_user(&repo).0;

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
    let users = create_users(&repo, n_articles)
        .into_iter()
        .map(|(u, _)| u)
        .collect();
    let articles = create_articles(&repo, users);

    let expected_tags: HashSet<String> = articles
        .into_iter()
        .map(|a| a.tag_list.into_iter())
        .flatten()
        .collect();

    let tags = articles::tags(&repo).unwrap();

    assert_eq!(expected_tags, tags);
}
