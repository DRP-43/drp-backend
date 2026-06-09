-- This file should undo anything in `up.sql`
ALTER TABLE "recipes" DROP IF EXISTS "ingredients";
ALTER TABLE "recipes" ADD COLUMN "ingredients" TEXT[] NOT NULL DEFAULT ARRAY['bad'] check(
    "ingredients" <> '{}' and
    array_position("ingredients", null) is null
);
