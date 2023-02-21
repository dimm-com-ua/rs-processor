drop index pc_task_worker_flow_udx;

create unique index pc_task_worker_flow_udx
    on pc_task_worker (element_id, task_id, what);