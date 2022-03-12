CREATE TABLE public.person
(
    id uuid NOT NULL,
    login character varying(32) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.person
    OWNER to sevenx_1_todo;