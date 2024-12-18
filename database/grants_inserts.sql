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

-- Gamba API
insert into grants (name)
values ('svc::pokemon_gamba::route::/pokemon/getRandomPokemon'),
       ('svc::pokemon_gamba::route::/pokemon/getUserGamba')
on conflict do nothing;

insert into grants (name)
values ('svc::pokemon_gamba::all_routes')
on conflict do nothing;
call link_grants('svc::pokemon_gamba::all_routes',
                 array ['svc::pokemon_gamba::route::/pokemon/getRandomPokemon', 'svc::pokemon_gamba::route::/pokemon/getUserGamba']);

-- Inventory API
insert into grants (name)
values ('svc::inventory_api::route::/pokemon/saveGamba'),
       ('svc::inventory_api::route::/pokemon/changeOwner'),
       ('svc::inventory_api::route::/pokemon/getInventory')
on conflict do nothing;

insert into grants (name)
values ('svc::inventory_api::all_routes')
on conflict do nothing;
call link_grants('svc::inventory_api::all_routes',
                 array ['svc::inventory_api::route::/pokemon/saveGamba', 'svc::inventory_api::route::/pokemon/changeOwner', 'svc::inventory_api::route::/pokemon/getInventory']);

-- Trading API
insert into grants (name)
values ('svc::trading_api::route::/pokemon/trade'),
       ('svc::trading_api::route::/pokemon/tradeHistory')
on conflict do nothing;

insert into grants (name)
values ('svc::trading_api::all_routes')
on conflict do nothing;
call link_grants('svc::trading_api::all_routes',
                 array ['svc::trading_api::route::/pokemon/trade', 'svc::trading_api::route::/pokemon/tradeHistory']);

-- Leaderboards API
insert into grants (name)
values ('svc::leaderboards_api::route::/pokemon/getLeaderboards')
on conflict do nothing;

insert into grants (name)
values ('svc::leaderboards_api::all_routes')
on conflict do nothing;
call link_grants('svc::leaderboards_api::all_routes',
                 array ['svc::leaderboards_api::route::/pokemon/getLeaderboards']);

-- Money manager API
insert into grants (name)
values ('svc::money_manager::route::/findUserWallet'),
       ('svc::money_manager::route::/modifyBalance')
on conflict do nothing;

insert into grants (name)
values ('svc::money_manager::all_routes')
on conflict do nothing;
call link_grants('svc::money_manager::all_routes',
                 array ['svc::money_manager::route::/findUserWallet', 'svc::money_manager::route::/modifyBalance']);

-- User info API
insert into grants (name)
values ('svc::user_info::route::/addUserInfo'),
       ('svc::user_info::route::/findUserInfo'),
       ('svc::user_info::route::/editUserInfo'),
       ('svc::user_info::route::/findAllUserInfo'),
       ('svc::user_info::route::/findUserInfoByUsername')
on conflict do nothing;

insert into grants (name)
values ('svc::user_info::all_routes')
on conflict do nothing;
call link_grants('svc::user_info::all_routes',
                 array ['svc::user_info::route::/addUserInfo', 'svc::user_info::route::/findUserInfo', 'svc::user_info::route::/editUserInfo', 'svc::user_info::route::/findAllUserInfo', 'svc::user_info::route::/findUserInfoByUsername']);

-- Payment gateway API
insert into grants (name)
values ('svc::payment_gateway:route:/createPayment'),
       ('svc::payment_gateway::route::/executePayment'),
       ('svc::payment_gateway::route::/paymentHistory')
on conflict do nothing;

insert into grants (name)
values ('svc::payment_gateway::all_routes')
on conflict do nothing;
call link_grants('svc::payment_gateway::all_routes',
                 array ['svc::payment_gateway:route:/createPayment', 'svc::payment_gateway::route::/executePayment', 'svc::payment_gateway::route::/paymentHistory']);
