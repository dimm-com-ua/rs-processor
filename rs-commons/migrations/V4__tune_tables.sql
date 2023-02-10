alter table pc_process_definition add column code varchar(50) not null ;

create unique index pc_process_definition_code_uindex on pc_process_definition(code);
create index pc_process_definition_code_index on pc_process_definition(code);