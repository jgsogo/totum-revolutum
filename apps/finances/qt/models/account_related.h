#pragma once

#include <QAbstractTableModel>
#include <QDate>
#include <QFont>
#include <QTimer>
#include <magic_enum/magic_enum.hpp>

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/accounts/cpp/models/types/money.h"

template <typename TModel> class AccountRelatedModel : public QAbstractTableModel {

    // Prepared to work with Snapshot and Movement
    static_assert(std::is_same_v<TModel, finances::accounts::models::Snapshot> ||
                  std::is_same_v<TModel, finances::accounts::models::Movement>);

  public:
    enum Column {
        ID = 0,
        DATE_VALUE = 1,
        AMOUNT = 2,
    };

  public:
    AccountRelatedModel(utils::libpqxx::ConnectionPool& pool_, const finances::accounts::models::Account& account_,
                        QObject* parent = nullptr)
        : QAbstractTableModel(parent), pool{pool_}, account{account_} {
        QTimer::singleShot(0, this, &AccountRelatedModel::fetch_all);
    };

    int rowCount(const QModelIndex& parent = QModelIndex()) const override { return items.size(); };

    int columnCount(const QModelIndex& parent = QModelIndex()) const override {
        return magic_enum::enum_count<Column>();
    }

    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override {
        QVariant result = QVariant();

        if (role == Qt::DisplayRole && orientation == Qt::Horizontal) { // H
            Column column = magic_enum::enum_value<Column>(section);
            result = QString::fromStdString(std::string(magic_enum::enum_name(column)));
        } else if (role == Qt::DisplayRole && orientation == Qt::Vertical) { // V
            return QString("%1").arg(items[section].id);
        } else {
            // other stuff
        }
        return result;
    }

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override {
        QVariant result = QVariant();

        int row = index.row();
        int column_idx = index.column();

        if (!index.isValid() || row >= rowCount() || column_idx >= columnCount()) {
            return result;
        }

        Column column = magic_enum::enum_value<Column>(column_idx);

        switch (role) {
        case Qt::DisplayRole: {
            const auto& item = items.at(row);
            switch (column) {
            case Column::ID:
                result = (uint64_t)item.id; // FIXME: implement the right conversion
                break;
            case Column::DATE_VALUE:
                result = QDate{int(item.date_value.year()), static_cast<int>(unsigned(item.date_value.month())),
                               static_cast<int>(unsigned(item.date_value.day()))}
                             .toString("yyyy-MM-dd");
                break;
            case Column::AMOUNT:
                auto amount_money = finances::accounts::models::Money{item.amount, account.ccy};
                result = QString::fromStdString(static_cast<std::string>(amount_money));
                break;
            }
        } break;
        case Qt::FontRole:
            if ((column == Column::DATE_VALUE) || (column == Column::AMOUNT)) {
                result = QFont{"Andale Mono"};
            }
            break;
        case Qt::TextAlignmentRole:
            result = Qt::AlignRight;
            break;
        default:
            break;
        }

        return result;
    }

  protected:
    void fetch_all() {
        SPDLOG_DEBUG("AccountRelatedModel<TModel>::fetch_all");

        typename TModel::Manager manager{pool};
        auto all_items = manager.all(account.id);
        if (!all_items) {
            SPDLOG_ERROR("Error refreshing items");
            // TODO: Communicate error to user
            return;
        }

        this->beginResetModel();
        this->items = std::move(all_items.value());
        this->endResetModel();
    }

  private:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    std::vector<TModel> items;
};
