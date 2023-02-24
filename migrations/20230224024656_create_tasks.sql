-- Add migration script here
create table tasks (
  id bigserial primary key,
  title text not null,
  done bool not null default 'f',
  created_at timestamptz not null default now()
);
