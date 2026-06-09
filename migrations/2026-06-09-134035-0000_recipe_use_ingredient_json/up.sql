-- Your SQL goes here
ALTER TABLE "recipes" DROP IF EXISTS "ingredients";
ALTER TABLE "recipes" ADD COLUMN "ingredients" JSONB[] NOT NULL DEFAULT ARRAY['{"name": "bad", "quantity": 0, "unit": "x", "category_id": "other"}'::jsonb] check(
    "ingredients" <> '{}' and
    array_position("ingredients", null) is null
);
