CREATE TABLE favorites (
    user_id UUID NOT NULL,
    article_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (article_id) REFERENCES articles(slug) ON DELETE CASCADE,
    PRIMARY KEY (user_id, article_id)
);
