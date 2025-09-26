#pragma once

#include "account_related.h"

class AccountRelatedSnapshotsModel : public AccountRelatedModel<finances::accounts::models::Snapshot> {
    Q_OBJECT
  public:
    // enum class Column {
    //     ID = 0,
    //     DATE_VALUE = 1,
    //     AMOUNT = 2,
    // };

  public:
    using AccountRelatedModel::AccountRelatedModel;

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
  private slots:
    void fetch_all_snapshots();
};
