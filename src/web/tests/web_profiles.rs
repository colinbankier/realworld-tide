// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::create_users;
use helpers::test_server::TestApp;

use async_std::task;
use itertools::Itertools;
use realworld_web::auth::encode_token;

#[test]
fn profiles_api() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let users = create_users(&server.repository.0, 2)
            .into_iter()
            .map(|(u, _)| u)
            .collect_vec();
        let follower_user = users[0].clone();
        let followed_user = users[1].clone();

        let followed_profile = server
            .get_profile(&followed_user.username, None)
            .await
            .unwrap();
        assert_eq!(followed_profile.profile.username, followed_user.username);
        assert_eq!(followed_profile.profile.bio, followed_user.bio);
        assert_eq!(followed_profile.profile.image, followed_user.image);
        assert_eq!(followed_profile.profile.following, false);

        let follower_token = encode_token(follower_user.id);
        let followed_profile = server
            .follow_profile(&followed_user.username, &follower_token)
            .await
            .unwrap();
        assert_eq!(followed_profile.profile.following, true);

        // If not logged in, following is still false
        let followed_profile = server
            .get_profile(&followed_user.username, None)
            .await
            .unwrap();
        assert_eq!(followed_profile.profile.following, false);

        // If logged in, following is correctly valued
        let followed_profile = server
            .get_profile(&followed_user.username, Some(&follower_token))
            .await
            .unwrap();
        assert_eq!(followed_profile.profile.following, true);

        let unfollowed_profile = server
            .unfollow_profile(&followed_user.username, &follower_token)
            .await
            .unwrap();
        assert_eq!(unfollowed_profile.profile.following, false);

        // After unfollowing, following is now false
        let p = server
            .get_profile(&followed_user.username, Some(&follower_token))
            .await
            .unwrap();
        assert_eq!(p.profile.following, false);
    })
}
