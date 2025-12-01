#include "qtableview_with_key_pressed.h"

#include <QKeyEvent>

void QTableViewWithKeyPressed::keyPressEvent(QKeyEvent* event) {
    // Emit the event with the selected indexes
    Qt::Key key = static_cast<Qt::Key>(event->key());
    QModelIndexList idxs = this->selectedIndexes();
    for (auto idx : idxs) {
        qDebug() << " - idx: " << idx;
        emit key_press_event(idx, key);
    }

    // Propagate
    QTableView::keyPressEvent(event);
}
