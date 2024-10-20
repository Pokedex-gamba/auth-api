-- Pokemon API
insert into grants (name)
values ('svc::pokemon_api::route::/pokemon/get_random'),
       ('svc::pokemon_api::route::/pokemon/get_all'),
       ('svc::pokemon_api::route::/pokemon/get_by_name')
on conflict do nothing;

-- Grouping
insert into grants (name)
values ('svc::pokemon_api::all_routes')
on conflict do nothing;
call link_grants('svc::pokemon_api::all_routes',
                 array ['svc::pokemon_api::route::/pokemon/get_random', 'svc::pokemon_api::route::/pokemon/get_all', 'svc::pokemon_api::route::/pokemon/get_by_name']);

-- Auth
insert into grants (name)
values ('svc::auth_api::route::/auth/register'),
       ('svc::auth_api::route::/auth/login')
on conflict do nothing;

insert into grants (name)
values ('svc::auth_api::all_routes')
on conflict do nothing;
call link_grants('svc::auth_api::all_routes',
                 array ['svc::auth_api::route::/auth/register', 'svc::auth_api::route::/auth/login']);


