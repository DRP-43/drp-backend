-- Your SQL goes here

ALTER TABLE "recipes" DROP IF EXISTS "ingredients";

CREATE TABLE "recipe_ingredients" (
    "recipe_id" INT8 NOT NULL,
    "name" VARCHAR(128) NOT NULL,
    "quantity" FLOAT8 NOT NULL,
    "unit" VARCHAR(8) NOT NULL,
    "category_id" VARCHAR(8) NOT NULL,
    PRIMARY KEY ("recipe_id", "name"),
    FOREIGN KEY ("recipe_id") REFERENCES "recipes" ("id")
)
