#pragma once

#include "account_related.h"

#include <QColor>

class AccountRelatedSnapshotsModel : public AccountRelatedModel<finances::accounts::models::Snapshot, SnapshotColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

  private slots:
    void fetch_all_snapshots();
};

class AccountRelatedSnapshotsAsMovementsModel
    : public AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

  private slots:
    void fetch_all_snapshots();
};

template <typename TColumn> struct DataDispatcher<finances::accounts::models::Snapshot, TColumn, Qt::BackgroundRole> {
    static QVariant data(const finances::accounts::models::Account&, const finances::accounts::models::Snapshot&,
                         TColumn column) {
        return QVariant{QColor(255, 255, 40)};
    }
};

template <>
QVariant DataDispatcher<finances::accounts::models::Snapshot, SnapshotColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account&, const finances::accounts::models::Snapshot&, SnapshotColumn);

template <>
QVariant DataDispatcher<finances::accounts::models::Snapshot, MovementColumn, Qt::DisplayRole>::data(
    const finances::accounts::models::Account&, const finances::accounts::models::Snapshot&, MovementColumn);
