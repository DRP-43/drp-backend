-- This file should undo anything in `up.sql`
ALTER TABLE "recipes" DROP COLUMN "is_public";
ALTER TABLE "recipes" DROP COLUMN "owner_id";
