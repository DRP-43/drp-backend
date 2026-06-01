CREATE TABLE "users" (
	"id" INT4 NOT NULL PRIMARY KEY
);

CREATE TABLE "recipes" (
	"id" INT4 NOT NULL PRIMARY KEY,
	"user_id" INT4 NOT NULL,
	"name" VARCHAR NOT NULL,
	"ingredients" TEXT[] NOT NULL
		check (
			"ingredients" <> '{}' and
			array_position("ingredients", null) is null
		),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);
