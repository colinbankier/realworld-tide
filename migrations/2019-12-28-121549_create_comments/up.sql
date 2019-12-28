CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    author_id UUID NOT NULL,
    article_id INTEGER NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);
