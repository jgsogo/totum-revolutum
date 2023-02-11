Totum revolutum
===============

> Loc. lat.; literalmente 'todo revuelto'.
> 
> m. revoltijo (‖ conjunto de cosas sin orden).

The aim of this project is to create an easy-to-use (git-like) tool to run sync
operations between different directories and storages. It uses an abstraction
over a filesystem and, on top of the abstraction, it builds some sync operations
like backup, copy, mirror,...

The project has evolved quite a bit and now it contains several crates:

 * [pcloud-sdk](pcloud_sdk): Implementation of the [pCloud](https://pcloud.com/) API
 * [filesystem](filesystem): Filesystem abstraction. It also implements `local`
   and `mock`.
 * [filesystem-pcloud](filesystem-pcloud): Implementation of the `filesystem` trait
   for remote pCloud storage.
 * [syncronia](sync): Operations between different filesystems. It also provides a 
   CLI tool
