-- Your SQL goes here

CREATE TABLE "users_queued_recipes" (
    "user_id" UUID NOT NULL,
    "recipe_id" UUID NOT NULL,
    "queue_number" INT4 NOT NULL,
    PRIMARY KEY ("user_id", "recipe_id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipes" ("id")
)
