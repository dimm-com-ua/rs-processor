alter table pc_process_flow_element_argument
    add constraint pc_process_flow_element_argument_pc_process_flow_element_id_fk
        foreign key (flow_element_id) references pc_process_flow_element;

