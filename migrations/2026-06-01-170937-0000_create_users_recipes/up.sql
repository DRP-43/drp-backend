CREATE TABLE "users" (
	"id" UUID NOT NULL PRIMARY KEY,
	"__is_dev" BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE "recipes" (
	"id" UUID NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"name" VARCHAR NOT NULL,
	"ingredients" TEXT[] NOT NULL
		check (
			"ingredients" <> '{}' and
			array_position("ingredients", null) is null
		),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);
