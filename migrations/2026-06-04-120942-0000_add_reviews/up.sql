-- Your SQL goes here

CREATE TABLE "recipe_reviews" (
    "user_id" UUID NOT NULL,
    "recipe_id" UUID NOT NULL,
    "rating" INT4 NOT NULL CHECK (0 <= "rating" AND "rating" <= 5),
    PRIMARY KEY ("user_id", "recipe_id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipes" ("id")
)
