

#pragma once

#include "libraries/utils/cpp/qt/models/generic_table_model.h"

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

template <typename Columns>
using SnapshotsTableModel = utils::qt::models::FilteredTableModel<finances::accounts::models::Account,
                                                                  finances::accounts::models::Snapshot, Columns>;

template <typename Columns>
using NumerableSnapshotsTableModel =
    utils::qt::models::FilteredTableModel<finances::accounts::models::Account,
                                          finances::investments::models::SnapshotNumerable, Columns>;
