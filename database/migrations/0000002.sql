CREATE TABLE public.task
(
    id bigint NOT NULL,
    text text NOT NULL,
    personid uuid NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (personid)
        REFERENCES public.person (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

ALTER TABLE IF EXISTS public.task
    OWNER to sevenx_1_todo;