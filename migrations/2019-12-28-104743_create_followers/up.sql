CREATE TABLE followers (
   followed_id UUID NOT NULL,
   follower_id UUID NOT NULL,
   FOREIGN KEY (followed_id) REFERENCES users(id) ON DELETE CASCADE,
   FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
   PRIMARY KEY (followed_id, follower_id)
);
