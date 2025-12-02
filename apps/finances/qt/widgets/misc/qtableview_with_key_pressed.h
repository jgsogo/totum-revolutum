#pragma once

#include <QTableView>

class QTableViewWithKeyPressed : public QTableView {
    Q_OBJECT
  public:
    using QTableView::QTableView;

  signals:
    void key_press_event(const QModelIndex& index, Qt::Key);

  protected:
    void keyPressEvent(QKeyEvent* event) override;
};
