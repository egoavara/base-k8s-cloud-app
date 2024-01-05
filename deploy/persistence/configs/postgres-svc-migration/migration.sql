create extension if not exists btree_gist;

create table if not exists asset
(
    id      uuid primary key default gen_random_uuid(),
    barcode varchar not null,
    name    varchar not null

    );

create table if not exists server
(
    id       uuid    not null,
    type     varchar not null,
    os       varchar not null,
    hostname varchar not null,
    domain   varchar not null,
    foreign key (id) references asset (id)
    );

create table if not exists cia
(
    id              uuid not null,
    confidentiality int4 not null default 1,
    integrity       int4 not null default 1,
    availability    int4 not null default 1,
    check ( confidentiality > 0 ),
    check ( integrity > 0 ),
    check ( availability > 0 ),
    foreign key (id) references asset (id)
    );


create table if not exists idc_building
(
    building varchar primary key
);

create table if not exists idc_floor
(
    building varchar,
    floor    varchar,
    primary key (building, floor),
    foreign key (building) references idc_building (building)
    );

create table if not exists idc_rack
(
    building varchar,
    floor    varchar,
    x        int4      not null,
    y        int4      not null,
    z_range  int4range not null,
    primary key (building, floor, x, y),
    foreign key (building, floor) references idc_floor (building, floor)
    );

create table if not exists location
(
    id      uuid      not null,
    b       varchar   not null,
    f       varchar   not null,
    x       int       not null,
    y       int       not null,
    z_range int4range not null,
    foreign key (id) references asset (id),
    foreign key (b, f, x, y) references idc_rack (building, floor, x, y),
    exclude using gist(b with =, f with =, x with =, y with =, z_range with &&)
    );