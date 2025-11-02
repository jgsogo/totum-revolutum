#pragma once

#include <spdlog/spdlog.h>

#include <QComboBox>
#include <QCompleter>
#include <QSortFilterProxyModel>

namespace utils::qt::widgets {

    class ComboBoxWithSearch : public QComboBox {
        Q_OBJECT

      public:
        ComboBoxWithSearch(QAbstractItemModel* model, QWidget* parent = nullptr);

        void setModelColumn(int column);

        QSortFilterProxyModel* sortFilterProxyModel();
        QCompleter* completer();

      protected:
        QSortFilterProxyModel* _sort_filter;
        QCompleter* _completer;
    };

} // namespace utils::qt::widgets
