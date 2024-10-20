create or replace view grants_with_subgrants as
select id, name,
       (with recursive grants_rec (id) as (
           -- anchor member
           select a_grants.id
           from grants as a_grants
           where a_grants.name = grants.name

           union

           -- recursive term
           select grants_to_grants.child_id
           from grants_rec,
                grants_to_grants
           where grants_to_grants.parent_id = grants_rec.id)

        select json_agg(name)
        from grants_rec
                 join grants as j_grants on grants_rec.id = j_grants.id) as all_grants
from grants
group by id, name