Totum revolutum
===============

> Loc. lat.; literalmente 'todo revuelto'.
>
> m. revoltijo (‖ conjunto de cosas sin orden).

I find myself reinventing the wheel from time to time, I find myself loving it! But many times
I find myself reinventing the wheel so I can reinvent the wheel again. There are a lot of tooling
and basic functionalities that every project needs before starting to write actual application
code: setting up the repo, basic CI, issues, testing, build-system, docs,... The purpose of
this repository is to gather together many (all?) my projects so they share common
foundations and I can focus and enjoy the development work itself.

Here you will find a little bit of everything. I'll try to do my best to keep it organized and
in well shape. Maybe, in the future, if something is worth it I'll extract it to a dedicated
repository. But right now, this mono repository looks like the best approach for myself.

Enjoy the visit!

## Applications

Applications in this repository:

## Libraries

You can find the following libraries in this repository:

## Tooling

Tools that can be reused from this repository:


[![Build and test](https://github.com/jgsogo/totum-revolutum/actions/workflows/bazel-diff.yaml/badge.svg)](https://github.com/jgsogo/totum-revolutum/actions/workflows/bazel-diff.yaml)

---


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
 * [syncronia](syncronia): Operations between different filesystems. It also provides a
   CLI tool
