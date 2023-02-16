delete from pc_task_worker;

alter table pc_task_worker add column element_id uuid not null ,
    add foreign key (element_id) references pc_process_definition_flow(id);

create unique index pc_task_worker_flow_udx on pc_task_worker (element_id, task_id);