CREATE TABLE IF NOT EXISTS public.person
(
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    login character varying(32) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT person_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.person
    OWNER to sevenx_1_todo;