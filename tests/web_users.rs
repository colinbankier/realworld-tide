// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::generate;
use helpers::test_server::TestApp;

use async_std::task;
use realworld_tide::db::models::UpdateUser;
use realworld_tide::web::users::responses::UserResponse;
use realworld_tide::web::users::update::UpdateUserRequest;

#[test]
fn register_and_login() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = generate::new_user();

        server.register_user(&user).await.unwrap();
        let token = server.login_user(&user).await.unwrap().user.token;
        let user_details = server.get_current_user(&token).await.unwrap();

        assert_eq!(user_details.user.username, user.username);
        assert_eq!(user_details.user.email, user.email);
    })
}

#[test]
fn update_and_retrieve_user_details() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = generate::new_user();

        let stored_user = server.register_user(&user).await.unwrap();
        let token = server.login_user(&user).await.unwrap().user.token;

        assert_eq!(stored_user.user.bio, None);
        assert_eq!(stored_user.user.image, None);

        let new_details = UpdateUserRequest {
            user: UpdateUser {
                bio: Some("I like to code.".to_string()),
                image: Some(
                    "https://www.rust-lang.org/static/images/rust-logo-blk.svg".to_string(),
                ),
                email: None,
                password: None,
                username: None,
            },
        };
        let updated_user: UserResponse = server
            .update_user_details(&new_details, &token)
            .await
            .unwrap();
        assert_eq!(updated_user.user.bio, new_details.user.bio);
        assert_eq!(updated_user.user.image, new_details.user.image);

        let current_user: UserResponse = server.get_current_user(&token).await.unwrap();
        assert_eq!(current_user.user.bio, new_details.user.bio);
        assert_eq!(current_user.user.image, new_details.user.image);
    })
}
