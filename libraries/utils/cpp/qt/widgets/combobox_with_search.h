#pragma once

#include <QComboBox>
#include <QCompleter>
#include <QHeaderView>
#include <QStandardItemModel>
#include <QTreeView>

#include "libraries/utils/cpp/expected_type.hpp"
#include "libraries/utils/cpp/qt/models/generic_table_model.h"

namespace utils::qt::widgets {

    class _ComboBoxWithSearch : public QComboBox {
        Q_OBJECT

      public:
        using QComboBox::QComboBox;

      signals:
        void activated(utils::db::Id);
    };

    template <class TModel, typename TColumn> class ComboBoxWithSearch : public _ComboBoxWithSearch {
        const static int ID_COLUMN = 1;
        const static int DISPLAY_COLUMN = 0;

      public:
        ComboBoxWithSearch(utils::qt::models::TableModel<TModel, TColumn>& model, TColumn col_id, TColumn col_display,
                           QWidget* parent = nullptr)
            : _ComboBoxWithSearch{parent}, _model{model} {
            this->setEditable(true);
            this->setFocusPolicy(Qt::StrongFocus);

            QCompleter* completer = new QCompleter(this);
            completer->setCaseSensitivity(Qt::CaseInsensitive);
            completer->setFilterMode(Qt::MatchContains);

            {
                // Create a model with all rows but only the selected columns
                QStandardItemModel* m = new QStandardItemModel{_model.rowCount(), 3, completer};
                for (int row = 0; row < _model.rowCount(); ++row) {
                    QModelIndex original_idx_id = _model.index(row, magic_enum::enum_integer(col_id));
                    QModelIndex original_idx_display = _model.index(row, magic_enum::enum_integer(col_display));

                    QModelIndex target_idx_display = m->index(row, DISPLAY_COLUMN);
                    QModelIndex target_idx_id = m->index(row, ID_COLUMN);

                    m->setData(target_idx_display, _model.data(original_idx_display));
                    m->setData(target_idx_id, _model.data(original_idx_id));
                }
                m->sort(0);
                completer->setModel(m);
                this->setModel(m);
            }

            {
                // Create a table-like view for the completer
                QTreeView* treeView = new QTreeView;
                completer->setPopup(treeView);
                treeView->hideColumn(ID_COLUMN);
                treeView->setRootIsDecorated(false);
                treeView->header()->hide();
                treeView->header()->setStretchLastSection(false);
                treeView->header()->setSectionResizeMode(0, QHeaderView::Stretch);
                // treeView->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
            }

            this->setCompleter(completer);

            connect(this, &QComboBox::activated, [this](const int index) {
                SPDLOG_DEBUG("ComboBoxWithSearch::activated(index={})", index);

                QModelIndex idIdx = this->model()->index(index, ID_COLUMN);
                QVariant item_id = this->model()->data(idIdx);

                auto id = item_id.value<utils::db::Id>();

                emit this->activated(id);
            });
        };

        ~ComboBoxWithSearch() = default;

        utils::ExpectedType<std::optional<std::reference_wrapper<const TModel>>, utils::qt::models::ErrorItemNotFound>
        selected() const {
            auto current_index = this->currentIndex();
            if (current_index == -1) {
                return {std::nullopt};
            }

            QModelIndex idIdx = this->model()->index(current_index, ID_COLUMN);
            QVariant item_id = this->model()->data(idIdx);

            auto id = item_id.value<utils::db::Id>();
            auto item_expected = _model.get(id);
            if (!item_expected) {
                return tl::unexpected{item_expected.error()};
            } else {
                return std::make_optional(std::move(item_expected.value()));
            }
        }

      private:
        const utils::qt::models::TableModel<TModel, TColumn>& _model;
    };

} // namespace utils::qt::widgets
