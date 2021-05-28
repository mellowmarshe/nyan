-- Add migration script here

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'Color') THEN
        CREATE TYPE Color AS ENUM ('brown', 'grey', 'white');
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'Area') THEN
        CREATE TYPE Area AS ENUM ('replenish', 'battle', 'rest');
    END IF;
    --more types here...
END$$;

CREATE TABLE IF NOT EXISTS public.cats (
    "id" BIGSERIAL PRIMARY KEY,
    "owner" BIGINT NOT NULL,
    "name" VARCHAR(32) UNIQUE NOT NULL,
    "color" Color NOT NULL,
    "area" Area NOT NULL DEFAULT 'rest', 
    "treats" BIGINT NOT NULL DEFAULT 10,
    "heterochromia" BOOLEAN DEFAULT false,
    "created" TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'utc')
);