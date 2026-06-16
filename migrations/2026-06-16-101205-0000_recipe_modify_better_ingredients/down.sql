-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "recipe_ingredients";

ALTER TABLE "recipes" ADD COLUMN IF NOT EXISTS "ingredients" JSONB[] NOT NULL
    DEFAULT ARRAY['{"name": "bad", "quantity": 0, "unit": "x", "category_id": "other"}'::jsonb]
    check(
        "ingredients" <> '{}' and
        array_position("ingredients", null) is null
    );
