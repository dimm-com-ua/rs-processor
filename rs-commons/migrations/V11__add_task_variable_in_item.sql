alter table pc_task_variable
    add flow_element_id uuid;

alter table pc_task_variable
    add constraint pc_task_variable_pc_process_flow_element_id_fk
        foreign key (flow_element_id) references pc_process_flow_element;