ALTER TABLE pc_process_definition_flow
    add column version_id int default 0 not null,
    add column created_at timestamptz null;

create table pc_handler_type
(
    id serial primary key not null ,
    name varchar(40) not null unique
);

create table pc_process_flow_element
(
    id uuid primary key not null default uuid_generate_v4(),
    process_flow uuid,
    constraint fk_process_flow
        foreign key (process_flow) references pc_process_definition_flow(id),
    type varchar(20),
    handler_type int constraint fk_handler_type references pc_handler_type(id),
    handler_value json not null  default '{}'
);

create unique index handler_type_name_uindex on pc_handler_type(name);
create index pc_process_element_type_idx on pc_process_flow_element(type);

insert into pc_handler_type (name) values ('starting'), ('on_event'), ('on_message'), ('continue'), ('match'), ('error'), ('end');