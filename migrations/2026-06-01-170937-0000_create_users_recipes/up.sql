CREATE TABLE "users" (
	"id" INT8 NOT NULL PRIMARY KEY,
	"__is_dev" BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE "recipes" (
	"id" INT8 NOT NULL PRIMARY KEY,
	"user_id" INT8 NOT NULL,
	"name" VARCHAR NOT NULL,
	"ingredients" TEXT[] NOT NULL
		check (
			"ingredients" <> '{}' and
			array_position("ingredients", null) is null
		),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);
