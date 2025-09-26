#pragma once

#include "account_related.h"
#include "account_related_movement.h"

enum class SnapshotColumn {
    ID = 0,
    DATE_VALUE = 1,
    AMOUNT = 2,
};

class AccountRelatedSnapshotsModel : public AccountRelatedModel<finances::accounts::models::Snapshot, SnapshotColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
  private slots:
    void fetch_all_snapshots();
};

class AccountRelatedSnapshotsAsMovementsModel
    : public AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn> {
    Q_OBJECT
  public:
  public:
    using AccountRelatedModel::AccountRelatedModel;

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
  private slots:
    void fetch_all_snapshots();
};

template <>
QVariant AccountRelatedModel<finances::accounts::models::Snapshot, MovementColumn>::data_display_role(
    MovementColumn column, const finances::accounts::models::Snapshot& item) const;
