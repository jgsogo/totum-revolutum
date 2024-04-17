
syncronia
=========

> Del fr. synchronie, y este del gr. σύγχρονος sýnchronos 'contemporáneo' e -ie '-ía'.
>
> 1. f. Coincidencia de hechos o fenómenos en el tiempo.
>
> <small>REAL ACADEMIA ESPAÑOLA: Diccionario de la lengua española, 23.ª ed., [versión 23.7 en línea]. <https://dle.rae.es> 2024-04-17.</small>




The aim of this application is to create an easy-to-use (git-like) tool to run sync
operations between different directories and storages. It uses an abstraction
over a filesystem and, on top of the abstraction, it builds some sync operations
like backup, copy, mirror,...

The project has evolved quite a bit and now it contains several crates:

 * [pcloud-sdk](pcloud_sdk): Implementation of the [pCloud](https://pcloud.com/) API
 * [filesystem](filesystem): Filesystem abstraction. It also implements `local`
   and `mock`.
 * [filesystem-pcloud](filesystem-pcloud): Implementation of the `filesystem` trait
   for remote pCloud storage.
 * [syncronia](syncronia): Operations between different filesystems. It also provides a
   CLI tool
