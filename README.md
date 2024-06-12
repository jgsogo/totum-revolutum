Totum revolutum
===============

> Loc. lat.; literalmente 'todo revuelto'.
>
> m. revoltijo (‖ conjunto de cosas sin orden).
>
> <small>REAL ACADEMIA ESPAÑOLA: Diccionario de la lengua española, 23.ª
> ed., [versión 23.7 en línea]. <https://dle.rae.es> 2024-04-17.</small>


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

* [photodb](app/photodb/): application to backup photos in pCloud storage and access them
* [syncronia](apps/syncronia/): a tool to run sync operations between different
  directories and storages

## Libraries

You can find the following libraries in this repository:

* [constants](libraries/constants/): compile-time constants definition
* [cron](libraries/cron/): utilities related to cron expressions
* [filesystem](libraries/filesystem/): abstraction of filesystem and files. Implementation for many different storages.
* [pcloud_sdk](libraries/pcloud_sdk/): pCloud SDK.
* [rebrickable](libraries/rebrickable/): API for https://rebrickable.com/
* [utils](libraries/utils/): generic utilities used by several libraries

## Tooling

Tools that can be reused from this repository:


---

[![Build and test](https://github.com/jgsogo/totum-revolutum/actions/workflows/bazel-diff.yaml/badge.svg)](https://github.com/jgsogo/totum-revolutum/actions/workflows/bazel-diff.yaml)
