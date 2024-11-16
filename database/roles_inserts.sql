insert into grants (name)
values ('role::public');
call link_grants('role::public', array ['svc::auth_api::route::/auth/register', 'svc::auth_api::route::/auth/login']);

insert into grants (id, name)
values ('60069534-615f-42ad-8ace-73bb7536850b', 'role::user');
--call link_grants('role::user', array []);

insert into grants (id, name)
values ('9008cfed-dcb3-40dd-a800-9d60f63e11b5', 'role::gigachad')
on conflict do nothing;
call link_grants('role::gigachad',
                 array ['svc::pokemon_api::all_routes', 'svc::auth_api::all_routes', 'svc::pokemon_gamba::all_routes']);
