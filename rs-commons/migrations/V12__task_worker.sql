create table pc_task_worker
(
    id uuid primary key not null default uuid_generate_v4(),
    task_id uuid not null ,
    constraint fk_task_worker_task
        foreign key (task_id) references pc_task(id),
    created_at timestamptz not null ,
    run_after timestamptz null ,
    runner_key uuid null ,
    locked_by timestamptz null
);

create index pc_task_worker_idx on pc_task_worker(id);