#pragma once

#include <QComboBox>
#include <QCompleter>
#include <QSortFilterProxyModel>

namespace utils::qt::widgets {

    class ComboBoxWithSearch : public QComboBox {
        Q_OBJECT

      public:
        ComboBoxWithSearch(QAbstractItemModel* model, QWidget* parent = nullptr);

        void setModelColumn(int column);
        // void focusInEvent(QFocusEvent *event);
        // void focusOutEvent(QFocusEvent *event);

        QSortFilterProxyModel* sortFilterProxyModel();
        QCompleter* completer();

      protected:
        QSortFilterProxyModel* _sort_filter;
        QCompleter* _completer;
    };

} // namespace utils::qt::widgets
