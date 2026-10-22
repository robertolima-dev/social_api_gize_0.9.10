-- Migration: social_rules
-- Write your forward schema changes below (SQL-first, Postgres; see ADR-011).


-- One like per user and post, one follow per pair, one profile per user. A repeat answers 409.
ALTER TABLE likes ADD CONSTRAINT likes_post_user_unique UNIQUE (post_id, user_id);
ALTER TABLE follows ADD CONSTRAINT follows_pair_unique UNIQUE (follower_id, followee_id);
ALTER TABLE profiles ADD CONSTRAINT profiles_user_unique UNIQUE (user_id);

-- Deleting a post deletes its comments and likes, instead of being refused while they exist.
ALTER TABLE comments DROP CONSTRAINT comments_post_id_fkey,
    ADD CONSTRAINT comments_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE;
ALTER TABLE likes DROP CONSTRAINT likes_post_id_fkey,
    ADD CONSTRAINT likes_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE;
