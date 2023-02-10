CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS pc_process_definition
(
    id uuid primary key not null default uuid_generate_v4(),
    name varchar(200) not null ,
    enabled bool default false
);

CREATE TABLE IF NOT EXISTS pc_process_definition_flow
(
    id uuid primary key not null default uuid_generate_v4(),
    process_id uuid,
    constraint fk_process
        foreign key (process_id)
            references pc_process_definition(id)
);
