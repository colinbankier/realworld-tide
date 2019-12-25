// use tokio_async_await_test::async_test;

// #[async_test]
// async fn test_create_user() {
//     let repo = Repo::new();

//     let new_user = generate::new_user();
//     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

//     let results =
//        find(repo.clone(), user.id).await
//     ;
//     assert!(results.is_ok());
// }

// #[async_test]
// async fn test_authenticate_user() {
//     let repo = Repo::new();
//     // Create a new user
//     let new_user = generate::new_user();
//     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

//     // Check the user is in the database.
//     let results =
//        find_by_email_password(repo.clone(), user.email, user.password).await
//     ;
//     assert!(results.is_ok());
// }

// #[async_test]
// async fn test_update_user() {
//     let repo = Repo::new();
//     // Create a new user
//     let new_user = generate::new_user();
//     let user =  insert(repo.clone(), new_user).await.expect("Create user failed.");

//     let new_details = UpdateUser {
//         bio: Some(fake!(Lorem.paragraph(3, 5)).to_string()),
//         image: Some(fake!(Internet.domain_suffix).to_string()),
//         email: Some(fake!(Internet.free_email).to_string()),
//         ..Default::default()
//     };

//     // Update the user
//     let result =  update(repo.clone(), user.id, new_details.clone() ).await;
//     result.expect("Failed to update user");

//     // Check the user is updated in the database.
//     let updated_user =
//        find(repo.clone(), user.id).await

//     .expect("Failed to fetch user");
//     assert_eq!(updated_user.bio, new_details.bio);
// }
