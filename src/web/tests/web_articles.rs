// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::generate;
use helpers::test_server::TestApp;
use helpers::{create_article, create_articles, create_user, create_users};

use async_std::task;
use domain::articles::ArticleQuery;
use fake::fake;
use itertools::Itertools;
use realworld_web::articles::insert::NewArticleRequest;
use realworld_web::articles::update::UpdateArticleRequest;
use realworld_web::auth::encode_token;

#[test]
fn should_list_articles() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let users = create_users(&server.repository.0, 5)
            .into_iter()
            .map(|(u, _)| u)
            .collect_vec();
        create_articles(&server.repository.0, users);
        let articles = server.get_articles(None).await.unwrap().articles;
        assert_eq!(articles.len(), 5);
    })
}

#[test]
fn favorite_count_is_updated_correctly() {
    task::block_on(async move {
        let mut server = TestApp::new();

        let n_users = 5;
        let users = create_users(&server.repository.0, n_users)
            .into_iter()
            .map(|(u, _)| u)
            .collect_vec();

        let author = users[0].clone();
        let slug = create_article(&server.repository.0, &author).slug;

        let article = server.get_article(&slug, None).await.unwrap().article;
        assert_eq!(slug, article.slug);
        assert_eq!(article.favorites_count, 0);

        for (i, user) in users.iter().enumerate() {
            let token = encode_token(user.id);
            server.favorite_article(&slug, &token).await.unwrap();

            let a = server
                .get_article(&slug, Some(&token))
                .await
                .unwrap()
                .article;
            assert_eq!(a.favorites_count, (i + 1) as u64);
            assert!(a.favorited);
        }

        for user in &users {
            let token = encode_token(user.id);
            server.unfavorite_article(&slug, &token).await.unwrap();

            let a = server
                .get_article(&slug, Some(&token))
                .await
                .unwrap()
                .article;
            assert!(!a.favorited);
        }

        let article = server.get_article(&slug, None).await.unwrap().article;
        assert_eq!(article.favorites_count, 0);
    })
}

#[test]
fn should_get_articles_by_author() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let users = create_users(&server.repository.0, 5)
            .into_iter()
            .map(|(u, _)| u)
            .collect_vec();
        create_articles(&server.repository.0, users.clone());

        let author = users[0].clone();
        let query = ArticleQuery {
            author: Some(author.username),
            tag: None,
            favorited: None,
        };
        let articles = server.get_articles(Some(query)).await.unwrap().articles;

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
        let (new_user, password) = generate::new_user();
        let user = server
            .register_user(&new_user, &password)
            .await
            .expect("Failed to create user")
            .user;
        let token = user.token;

        let article = generate::article_content();
        let new_article_request = realworld_web::articles::insert::Request {
            article: NewArticleRequest {
                title: article.title.clone(),
                description: article.description.clone(),
                body: article.body.clone(),
                tag_list: Some(article.tag_list.clone()),
            },
        };
        server
            .create_article(&new_article_request, &token)
            .await
            .unwrap();

        let query = Some(ArticleQuery {
            author: Some(user.username),
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

#[test]
fn should_update_article() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = create_user(&server.repository.0).0;
        let token = encode_token(user.id);
        let article = create_article(&server.repository.0, &user);

        let update = realworld_web::articles::update::Request {
            article: UpdateArticleRequest {
                title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
                description: None,
                body: Some(fake!(Lorem.paragraph(10, 5))),
            },
        };
        let updated_article = server
            .update_article(&update, &article.slug, &token)
            .await
            .unwrap();
        assert_eq!(update.article.title, updated_article.article.title.into());
        assert_eq!(article.description, updated_article.article.description);
        assert_eq!(update.article.body, updated_article.article.body.into());
    })
}

#[test]
fn should_delete_article() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = create_user(&server.repository.0).0;
        let token = encode_token(user.id);
        let article = create_article(&server.repository.0, &user);

        server
            .get_article(&article.slug, Some(&token))
            .await
            .unwrap();

        server.delete_article(&article.slug, &token).await.unwrap();

        let result = server.get_article(&article.slug, Some(&token)).await;
        assert!(result.is_err());
    })
}
