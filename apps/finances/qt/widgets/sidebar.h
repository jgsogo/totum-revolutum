#pragma once

#include <QWidget>

class SideBar : public QWidget {
    Q_OBJECT
  public:
    explicit SideBar();

    // void addAction(QAction *action);
    // QAction *addAction(const QString &text, const QIcon &icon = QIcon());
    // QSize minimumSizeHint() const;

    // protected:
    //     void paintEvent(QPaintEvent *event);
    //     void mousePressEvent(QMouseEvent *event);
    //     void mouseMoveEvent(QMouseEvent *event);
    //     void leaveEvent(QEvent * event);

    //     QAction *actionAt(const QPoint &at);
  private:
    // QList<QAction *> mActions;

    // QAction *mCheckedAction;
    // QAction *mOverAction;
};
