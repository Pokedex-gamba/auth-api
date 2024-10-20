create or replace procedure link_grants(_parent text, _children text[])
    language plpgsql
as
$$
begin
    insert into grants_to_grants (parent_id, child_id)
    select (select id from grants where name = _parent), (select id from grants where name = child_id__)
    from unnest(_children) as child_id__ on conflict do nothing;
end;
$$;
