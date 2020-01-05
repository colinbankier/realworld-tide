mod helpers;

use crate::helpers::create_user;
use helpers::generate;
use helpers::test_db::get_test_repo;
use realworld_tide::domain::repositories::{ArticleRepository, UsersRepository};
use realworld_tide::domain::PublishArticleError;

#[test]
fn slugs_must_be_unique() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let author = create_user(&repository.0);
    let author = repository.get_by_id(author.id).unwrap();
    let first_draft = generate::article_content();
    let second_draft = first_draft.clone();
    // Two article drafts, with identical title => identical slug
    assert_eq!(first_draft.slug(), second_draft.slug());

    let expected_slug = first_draft.slug();

    let result = author.publish(first_draft, &repository);
    assert!(result.is_ok());

    // Publishing the second draft fails
    let result = author.publish(second_draft, &repository);
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
    let author = repository.get_by_id(author.id).unwrap();
    let draft = generate::article_content();

    let expected_article = author.publish(draft, &repository).unwrap();
    let retrieved_article = repository.get_by_slug(&expected_article.slug).unwrap();
    assert_eq!(expected_article, retrieved_article);
}
