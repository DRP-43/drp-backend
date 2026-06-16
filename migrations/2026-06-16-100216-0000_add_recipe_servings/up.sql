-- Your SQL goes here
ALTER TABLE "recipes" ADD "servings" INT4 NOT NULL DEFAULT 1 CHECK ("servings" >= 0);
