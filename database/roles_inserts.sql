insert into grants (name)
values ('role::public');
call link_grants('role::public', array ['svc::auth_api::route::/auth/register', 'svc::auth_api::route::/auth/login']);

insert into grants (id, name)
values ('60069534-615f-42ad-8ace-73bb7536850b', 'role::user');
call link_grants('role::user',
                 array ['svc::pokemon_api::route::/pokemon/get_random', 'svc::pokemon_api::route::/pokemon/get_all', 'svc::pokemon_api::route::/pokemon/get_by_name', 'svc::auth_api::route::/auth/register', 'svc::auth_api::route::/auth/login', 'svc::pokemon_gamba::route::/pokemon/getRandomPokemon', 'svc::pokemon_gamba::route::/pokemon/getUserGamba', 'svc::inventory_api::route::/pokemon/saveGamba', 'svc::inventory_api::route::/pokemon/changeOwner', 'svc::inventory_api::route::/pokemon/getInventory', 'svc::trading_api::route::/pokemon/trade', 'svc::trading_api::route::/pokemon/tradeHistory', 'svc::leaderboards_api::route::/pokemon/getLeaderboards', 'svc::money_manager::route::/findUserWallet', 'svc::money_manager::route::/modifyBalance', 'svc::user_info::route::/addUserInfo', 'svc::user_info::route::/findUserInfo', 'svc::user_info::route::/editUserInfo', 'svc::user_info::route::/findAllUserInfo', 'svc::user_info::route::/findUserInfoByUsername']);

insert into grants (id, name)
values ('9008cfed-dcb3-40dd-a800-9d60f63e11b5', 'role::gigachad')
on conflict do nothing;
call link_grants('role::gigachad',
                 array ['svc::pokemon_api::all_routes', 'svc::auth_api::all_routes', 'svc::pokemon_gamba::all_routes', 'svc::inventory_api::all_routes', 'svc::trading_api::all_routes', 'svc::leaderboards_api::all_routes', 'svc::money_manager::all_routes', 'svc::user_info::all_routes']);
