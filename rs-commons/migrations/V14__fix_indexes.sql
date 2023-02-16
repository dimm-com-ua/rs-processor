alter table pc_task_worker
    drop constraint pc_task_worker_element_id_fkey;

alter table pc_task_worker
    add foreign key (element_id) references pc_process_flow_element;
