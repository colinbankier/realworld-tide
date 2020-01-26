mod helpers;

use fake::fake;
use helpers::create_user;
use helpers::test_db::get_test_repo;
use realworld_tide::conduit::users;
use realworld_tide::db::models::UpdateUser;

#[test]
fn test_create_user() {
    let repo = get_test_repo();
    let user = create_user(&repo);
    let results = users::find(&repo, user.id);
    assert!(results.is_ok());
}

#[test]
fn test_authenticate_user() {
    let repo = get_test_repo();

    // Create a new user
    let user = create_user(&repo);

    // Check the user is in the database.
    let results = users::find_by_email_password(&repo, &user.email, &user.password);
    assert!(results.is_ok());
}

#[test]
fn test_update_user() {
    let repo = get_test_repo();
    // Create a new user
    let user = create_user(&repo);

    let new_details = UpdateUser {
        bio: Some(fake!(Lorem.paragraph(3, 5)).to_string()),
        image: Some(fake!(Internet.domain_suffix).to_string()),
        email: Some(fake!(Internet.free_email).to_string()),
        ..Default::default()
    };

    // Update the user
    let result = users::update(&repo, user.id, new_details.clone());
    result.expect("Failed to update user");

    // Check the user is updated in the database.
    let updated_user = users::find(&repo, user.id).expect("Failed to fetch user");
    assert_eq!(updated_user.bio, new_details.bio);
}
