mod helpers;

use crate::helpers::generate::With;
use crate::helpers::{create_article2, create_user, create_user2};
use db::Repository;
use fake::fake;
use helpers::generate;
use helpers::test_db::get_test_repo;
use realworld_domain::repositories::Repository as RepositoryTrait;
use realworld_domain::{ArticleUpdate, PublishArticleError};

#[test]
fn slugs_must_be_unique() {
    let repo = get_test_repo();
    let repository = Repository(repo);

    let author = create_user2(&repository).0;
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
    let repository = Repository(repo);

    let author = create_user(&repository.0).0;
    let author = repository.get_user_by_id(author.id).unwrap();
    let draft = generate::article_content();

    let expected_article = author.publish(draft, &repository).unwrap();
    let retrieved_article = repository
        .get_article_by_slug(&expected_article.slug)
        .unwrap();
    assert_eq!(expected_article, retrieved_article);
}

#[test]
fn update_and_retrieve_article() {
    let repo = get_test_repo();
    let repository = Repository(repo);

    let author = create_user2(&repository).0;
    let article = create_article2(&repository, With::Value(&author));

    let update = ArticleUpdate {
        title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
        description: Some(fake!(Lorem.paragraph(3, 10)).to_string()),
        body: Some(fake!(Lorem.paragraph(10, 5)).to_string()),
    };
    let updated_article = author
        .update_article(article, update.clone(), &repository)
        .unwrap();

    assert_eq!(update.title, updated_article.content.title.into());
    assert_eq!(
        update.description,
        updated_article.content.description.into()
    );
    assert_eq!(update.body, updated_article.content.body.into());
}
