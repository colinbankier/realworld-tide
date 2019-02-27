use diesel::prelude::*;
use diesel::result::Error;

use crate::db::Repo;
use crate::models::{Article, NewArticle};
use crate::schema::articles;

pub async fn insert(repo: Repo, article: NewArticle) -> Result<Article, Error> {
    await! { repo.run(move |conn| {
        diesel::insert_into(articles::table)
            .values(&article)
            .get_result(&conn)
    })}
}

pub async fn all(repo: Repo) -> Result<Vec<Article>, Error> {
    use crate::schema::articles::dsl::*;
    await! { repo.run(move |conn| articles.load(&conn)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{create_articles, create_users, init_env};
    use tokio_async_await_test::async_test;

    #[async_test]
    async fn test_list_articles() {
        init_env();
        let repo = Repo::new();

        let users = await! { create_users(&repo, 5) };
        let _articles = await! { create_articles(&repo, users)};
        let results = await! { all(repo.clone())}.expect("Failed to get articles");
        // let articles = await! {
        //     test_users.into_iter()
        //         .filter_map(|user_result| user_result.ok())
        //         .map(|user| insert(repo.clone(), generate::new_article(user.id)) )
        //         .collect::<FuturesOrdered<_>>().collect::<Vec<_>>()
        // };
        // println!("test articles {:?}", articles);
        // let results = articles
        //     .into_iter()
        //     .filter(|a| a.is_ok())
        //     .collect::<Vec<_>>();
        assert_eq!(results.len(), 5);
    }
}
