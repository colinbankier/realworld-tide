mod helpers;

use fake::fake;
use helpers::create_user;
use helpers::test_db::get_test_repo;
use realworld_db::models::UpdateUser;
use realworld_db::queries::users;

#[test]
fn test_create_user() {
    let repo = get_test_repo();
    let user = create_user(&repo).0;
    let results = users::find(&repo, user.id);
    assert!(results.is_ok());
}

#[test]
fn test_authenticate_user() {
    let repo = get_test_repo();

    // Create a new user
    let user = create_user(&repo).0;

    // Check the user is in the database.
    let results = users::find_by_email(&repo, &user.email);
    assert!(results.is_ok());
}

#[test]
fn test_update_user() {
    let repo = get_test_repo();
    // Create a new user
    let user = create_user(&repo).0;

    let bio = fake!(Lorem.paragraph(3, 5)).to_string();
    let image = fake!(Internet.domain_suffix).to_string();
    let email = fake!(Internet.free_email).to_string();
    let new_details = UpdateUser {
        bio: Some(&bio),
        image: Some(&image),
        email: Some(&email),
        ..Default::default()
    };

    // Update the user
    let result = users::update(&repo, user.id, new_details.clone());
    result.expect("Failed to update user");

    // Check the user is updated in the database.
    let updated_user = users::find(&repo, user.id).expect("Failed to fetch user");
    assert_eq!(updated_user.bio, Some(bio));
    assert_eq!(updated_user.image, Some(image));
    assert_eq!(updated_user.email, email);
}
