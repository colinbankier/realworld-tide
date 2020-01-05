mod helpers;

use crate::helpers::create_user;
use helpers::generate;
use helpers::generate::With;
use helpers::test_db::get_test_repo;
use realworld_tide::domain::repositories::ArticleRepository;
use realworld_tide::domain::PublishArticleError;

#[test]
fn cannot_publish_an_article_if_the_author_does_not_exist() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let draft = generate::article_draft(With::Random);
    let expected_author_id = draft.author_id.to_owned();

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

#[test]
fn slugs_must_be_unique() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let author = create_user(&repository.0);
    // Two article drafts, with identical title => identical slug
    let first_draft = generate::article_draft(With::Value(author.id));
    let second_draft = first_draft.clone();
    let expected_slug = first_draft.slug();

    let result = first_draft.publish(&repository);
    assert!(result.is_ok());

    // Publishing the second draft fails
    let result = second_draft.publish(&repository);
    assert!(result.is_err());

    // With the appropriate error variant
    match result.unwrap_err() {
        PublishArticleError::DuplicatedSlug { slug, source: _ } => assert_eq!(expected_slug, slug),
        _ => panic!("Unexpected error"),
    }
}

#[test]
fn insert_and_retrieve_article() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let author = create_user(&repository.0);
    let draft = generate::article_draft(With::Value(author.id));

    let expected_article = draft.publish(&repository).unwrap();
    let retrieved_article = repository.get_by_slug(&expected_article.slug).unwrap();
    assert_eq!(expected_article, retrieved_article);
}
