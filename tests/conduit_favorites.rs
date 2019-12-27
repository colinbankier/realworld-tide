mod helpers;

use helpers::test_db::get_test_repo;
use helpers::{create_article, create_user};

use realworld_tide::conduit::favorites;

#[test]
fn you_cannot_favorite_an_article_which_does_not_exist() {
    let repo = get_test_repo();

    let user = create_user(&repo);
    // Id not pointing to any article in the DB
    let article_id = 1;
    let result = favorites::favorite(&repo, user.id, article_id);
    assert!(result.is_err());
}

#[test]
fn you_can_favorite_an_article_twice() {
    let repo = get_test_repo();

    let user = create_user(&repo);
    let article = create_article(&repo, user.clone());

    let result = favorites::favorite(&repo, user.id, article.id);
    assert!(result.is_ok());

    let result = favorites::favorite(&repo, user.id, article.id);
    assert!(result.is_ok());
}
