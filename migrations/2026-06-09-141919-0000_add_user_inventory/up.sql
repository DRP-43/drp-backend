-- Your SQL goes here

CREATE TABLE "users_inventory" (
    "user_id" INT8 NOT NULL,
    "name" VARCHAR(128) NOT NULL,
    "quantity" FLOAT8 NOT NULL,
    "unit" VARCHAR(8) NOT NULL,
    "expiration_date" VARCHAR(12) DEFAULT NULL,
    "category_id" VARCHAR(8) NOT NULL,
    PRIMARY KEY ("user_id", "name"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id")
)
