// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::generate;
use helpers::test_server::TestApp;
use helpers::{create_article, create_articles, create_users};

use async_std::task;
use realworld_tide::auth::encode_token;
use realworld_tide::conduit::articles::ArticleQuery;
use realworld_tide::conduit::users;
use realworld_tide::web::articles::insert::NewArticleRequest;

#[test]
fn should_list_articles() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let users = create_users(&server.repository, 5);
        create_articles(&server.repository, users);
        let articles = server.get_articles(None).await.unwrap().articles;
        assert_eq!(articles.len(), 5);
    })
}

#[test]
fn favorite_count_is_updated_correctly() {
    task::block_on(async move {
        let mut server = TestApp::new();

        let n_users = 5;
        let users = create_users(&server.repository, n_users);

        let author = users[0].clone();
        let slug = create_article(&server.repository, author).slug;

        let article = server.get_article(&slug).await.unwrap().article;
        assert_eq!(slug, article.slug);
        assert_eq!(article.favorites_count, 0);

        for (i, user) in users.iter().enumerate() {
            let token = encode_token(user.id);
            server.favorite_article(&slug, &token).await.unwrap();

            let n_fav = server
                .get_article(&slug)
                .await
                .unwrap()
                .article
                .favorites_count;
            assert_eq!(n_fav, (i + 1) as u64);
        }
    })
}

#[test]
fn should_get_articles_by_author() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let users = create_users(&server.repository, 5);
        create_articles(&server.repository, users.clone());

        let author = users[0].clone();
        let query = Some(ArticleQuery {
            author: Some(author.username),
            tag: None,
            favorited: None,
        });
        let articles = server.get_articles(query).await.unwrap().articles;

        assert_eq!(articles.len(), 1);
        let retrieved_article = articles[0].clone();
        assert_eq!(retrieved_article.title, articles[0].title);
        assert_eq!(retrieved_article.description, articles[0].description);
        assert_eq!(retrieved_article.body, articles[0].body);
        assert_ne!(retrieved_article.slug, "");
    })
}

#[test]
fn should_create_article() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = generate::new_user();
        let author = users::insert(&server.repository, user).expect("Failed to create user");
        let token = encode_token(author.id.to_owned());

        let article = generate::new_article(author.id);
        let new_article_request = realworld_tide::web::articles::insert::Request {
            article: NewArticleRequest {
                title: article.title.clone(),
                description: article.description.clone(),
                body: article.body.clone(),
                tag_list: article.tag_list.clone(),
            },
        };
        server
            .create_article(&new_article_request, &token)
            .await
            .unwrap();

        let query = Some(ArticleQuery {
            author: Some(author.username),
            tag: None,
            favorited: None,
        });
        let articles = server.get_articles(query).await.unwrap().articles;

        assert_eq!(articles.len(), 1);
        let retrieved_article = articles[0].clone();
        assert_eq!(retrieved_article.title, article.title);
        assert_eq!(retrieved_article.description, article.description);
        assert_eq!(retrieved_article.body, article.body);
        assert_ne!(retrieved_article.slug, "");
    })
}
