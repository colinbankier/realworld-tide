mod helpers;

use helpers::generate;
use helpers::test_db::get_test_repo;

#[test]
#[should_panic]
fn cannot_publish_an_article_if_the_author_does_not_exist() {
    let repo = get_test_repo();
    let repository = realworld_tide::conduit::articles_repository::Repository(&repo);

    let draft = generate::article_draft(None);
    draft.publish(&repository);
}
