-- This file should undo anything in `up.sql`
ALTER TABLE "recipes" ADD "user_id" INT4 NOT NULL DEFAULT 0; 
