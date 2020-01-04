mod helpers;

use helpers::generate;
use helpers::test_db::get_test_repo;
use realworld_tide::domain::PublishArticleError;

#[test]
fn cannot_publish_an_article_if_the_author_does_not_exist() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let draft = generate::article_draft(None);
    let expected_author_id = draft.author_id().to_owned();

    let result = draft.publish(&repository);
    // Publish fails
    assert!(result.is_err());

    // With the appropriate error variant
    match result.unwrap_err() {
        PublishArticleError::AuthorNotFound {
            author_id,
            source: _,
        } => assert_eq!(expected_author_id, author_id),
        _ => panic!("Unexpected error"),
    }
}
