-- Your SQL goes here
ALTER TABLE "recipes" ADD "is_public" BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE "recipes" ADD "owner_id" INT8 NOT NULL DEFAULT 0;
ALTER TABLE "recipes" ADD FOREIGN KEY ("owner_id" ) REFERENCES "users" ("id");
