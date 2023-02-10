CREATE TABLE IF not exists pc_process_flow_route
(
    id uuid primary key not null default uuid_generate_v4(),
    process_flow uuid not null ,
    constraint fk_process_flow_route
        foreign key (process_flow) references pc_process_definition_flow(id),

    is_conditional bool default false,
    condition json default '{}',
    from_item uuid not null ,
    to_element uuid not null ,
    priority int default 0,
    description varchar(400),

    constraint fk_process_flow_element_from
        foreign key (from_item) references pc_process_flow_element(id),
    constraint fk_process_flow_element_to
        foreign key (to_element) references pc_process_flow_element(id)
);

create index if not exists pc_process_flow_route_idx on pc_process_flow_route(id);

alter table pc_process_flow_element
    add column if not exists description varchar(400);

create index if not exists pc_process_flow_element_idx on pc_process_flow_element(id);

create table if not exists pc_data_type
(
    id varchar(40) not null primary key ,
    name varchar(200) not null default '',
    handler varchar(40) not null default 'simple_value'
);

create index if not exists pc_data_type_idx on pc_data_type(id);
create unique index if not exists pc_data_type_id_uindex on pc_data_type(id);

-- insert into pc_data_type (id, name)
-- values ('string', 'String'),  ('number', 'Number'),  ('date', 'Date'), ('datetime', 'Date with Time'),  ('object', 'Object');

create table if not exists pc_process_flow_element_argument
(
    id uuid primary key not null default uuid_generate_v4(),
    process_flow uuid not null ,
    constraint fk_process_flow_element_argument
        foreign key (process_flow) references pc_process_definition_flow(id),
    name varchar(40) not null ,
    direction varchar(20) not null default 'in',
    data_type varchar(40) not null default 'string',
    is_required bool default false,

    constraint fk_process_flow_element_argument_data_type
        foreign key (data_type) references pc_data_type(id)
);

create index if not exists pc_process_flow_element_argument_idx on pc_process_flow_element_argument(id);
create unique index if not exists pc_process_flow_element_argument_arg on pc_process_flow_element_argument(process_flow, name, direction);

create index if not exists pc_handler_type_idx on pc_handler_type(id);
create index if not exists pc_process_definition_idx on pc_process_definition(id);
create index if not exists pc_process_definition_flow_idx on pc_process_definition_flow(id);
create index if not exists  pc_process_flow_element_idx on pc_process_flow_element(id);

create table if not exists pc_task
(
    id uuid primary key not null default uuid_generate_v4(),
    process_flow uuid not null ,
    constraint fk_task_process
        foreign key (process_flow) references pc_process_definition_flow(id),
    created_at timestamptz not null ,
    current_flow_item uuid not null
);

create index if not exists pc_task_idx on pc_task(id);

create table if not exists pc_task_variable
(
    id uuid primary key not null default uuid_generate_v4(),
    task_id uuid not null ,
    name varchar(40) not null ,
    constraint fk_task_variable_task
        foreign key (task_id) references pc_task(id),
    data_type varchar(40) not null default 'string',

    constraint fk_task_variable_data_type
        foreign key (data_type) references pc_data_type(id),

    value json default '{}' not null
);

create index if not exists pc_task_variable_idx on pc_task_variable(id);
create unique index if not exists pc_task_variable_uindex on pc_task_variable(task_id, name);