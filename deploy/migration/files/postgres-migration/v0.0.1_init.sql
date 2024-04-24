create table if not exists project
(
    project_id  uuid          not null default uuid_generate_v4(),
    name        varchar(1024) not null,
    description varchar       null,
    primary key (project_id)
);

create table if not exists otype
(
    otype_id    uuid          not null default uuid_generate_v4(),
    name        varchar(1024) not null,
    description varchar       null,
    definition  jsonb         not null,
    primary key (otype_id)

);

create table if not exists source
(
    source_id   uuid          not null default uuid_generate_v4(),
    name        varchar(1024) not null,
    description varchar       null,
    primary key (source_id)
);

create table if not exists value
(
    value_id    uuid    not null default uuid_generate_v4(),
    source_id   uuid    not null,
    user_name   varchar not null,
    relation    varchar not null,
    object_name varchar not null,

    primary key (value_id),
    foreign key (source_id) references source (source_id)
);

create table if not exists project_otype
(
    project_id uuid not null,
    otype_id   uuid not null,
    primary key (project_id, otype_id),
    foreign key (project_id) references project (project_id),
    foreign key (otype_id) references otype (otype_id)
);

create table if not exists project_source
(
    project_id      uuid not null,
    user_otype_id   uuid not null,
    object_otype_id uuid not null,
    source_id       uuid not null,
    primary key (project_id, user_otype_id, object_otype_id, source_id),
    foreign key (project_id, user_otype_id) references project_otype (project_id, otype_id),
    foreign key (project_id, object_otype_id) references project_otype (project_id, otype_id),
    foreign key (source_id) references source (source_id)
);

create table if not exists source_assignable
(
    source_id       uuid not null,
    user_otype_id   uuid not null,
    object_otype_id uuid not null,
    primary key (source_id, user_otype_id, object_otype_id),
    foreign key (source_id) references source (source_id),
    foreign key (user_otype_id) references otype (otype_id),
    foreign key (object_otype_id) references otype (otype_id)
);