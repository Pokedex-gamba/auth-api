create table grants
(
    id   uuid default gen_random_uuid() not null
        constraint grants_pk
            primary key,
    name text                           not null
        constraint grants_unique_name
            unique
);

create table users
(
    id         uuid      default gen_random_uuid()                            not null
        constraint users_pk
            primary key,
    email      text                                                           not null
        constraint users_unique_email
            unique,
    hash       bytea                                                          not null,
    salt       text                                                           not null,
    created_at timestamp default CURRENT_TIMESTAMP                            not null,
    "grant"    uuid      default '60069534-615f-42ad-8ace-73bb7536850b'::uuid not null
        constraint users_grants_id_fk
            references grants
);

create table grants_to_grants
(
    parent_id uuid not null
        constraint grants_to_grants_grants_id_fk
            references grants,
    child_id  uuid not null
        constraint grants_to_grants_grants_id_fk2
            references grants,
    constraint grants_to_grants_pk
        primary key (parent_id, child_id)
);
