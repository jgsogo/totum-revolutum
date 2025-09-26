#include "account_related_snapshot.h"

#include <QColor>

void AccountRelatedSnapshotsModel::fetch_all_snapshots() { this->fetch_all(); }

QVariant AccountRelatedSnapshotsModel::data(const QModelIndex& index, int role) const {
    // Snapshot override some properties of their cells

    if (role == Qt::BackgroundRole) {
        QVariant result = QColor(255, 255, 40);
        return result;
    }

    return AccountRelatedModel<finances::accounts::models::Snapshot>::data(index, role);
}
