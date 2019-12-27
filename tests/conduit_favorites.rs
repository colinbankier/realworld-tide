mod helpers;

use helpers::test_db::get_test_repo;
use helpers::{create_article, create_user, create_users};

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
fn you_can_favorite_an_article_twice_but_it_only_counts_for_one() {
    let repo = get_test_repo();

    let user = create_user(&repo);
    let article = create_article(&repo, user.clone());

    let result = favorites::favorite(&repo, user.id, article.id);
    assert!(result.is_ok());

    let result = favorites::favorite(&repo, user.id, article.id);
    assert!(result.is_ok());

    assert_eq!(1, favorites::n_favorites(&repo, article.id).unwrap());
    assert!(favorites::is_favorite(&repo, user.id, vec![article.id]).unwrap()[&article.id])
}

#[test]
fn favorites_works() {
    let repo = get_test_repo();

    let author = create_user(&repo);
    let article = create_article(&repo, author);

    let n_fans = 10;
    let fans = create_users(&repo, n_fans);
    for fan in &fans {
        assert!(!favorites::is_favorite(&repo, fan.id, vec![article.id]).unwrap()[&article.id]);
        favorites::favorite(&repo, fan.id, article.id).expect("Failed to fav article");
        assert!(favorites::is_favorite(&repo, fan.id, vec![article.id]).unwrap()[&article.id]);
    }

    assert_eq!(
        n_fans as i64,
        favorites::n_favorites(&repo, article.id).unwrap()
    );
}
