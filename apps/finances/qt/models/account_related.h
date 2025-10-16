#pragma once

#include <QAbstractTableModel>
#include <QColor>
#include <QDate>
#include <QFont>
#include <QTimer>
#include <magic_enum/magic_enum.hpp>

#include "libraries/finances/accounts/cpp/models/account.h"

#include "movement_type.h"

class AccountRelatedModelBase;

template <typename TModel, typename TColumn, enum Qt::ItemDataRole> struct DataDispatcher {
    static QVariant data(const AccountRelatedModelBase&, const TModel&, TColumn) { return QVariant{}; }
};

class AccountRelatedModelBase : public QAbstractTableModel {
    Q_OBJECT
  public:
    AccountRelatedModelBase(utils::libpqxx::ConnectionPool& pool_, const finances::accounts::models::Account& account_,
                            const MovementTypeTableModel* movtype_model_, QObject* parent = nullptr)
        : QAbstractTableModel(parent), pool{pool_}, account{account_}, movtype_model{movtype_model_} {
        QTimer::singleShot(0, this, &AccountRelatedModelBase::fetch_all);
    };

    template <typename TTModel, typename TTColumn, enum Qt::ItemDataRole> friend struct DataDispatcher;

  public slots:
    void fetch_all() { this->_fetch_all(); };

  protected:
    virtual void _fetch_all() = 0;

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Account& account;
    const MovementTypeTableModel* movtype_model;
};

template <typename TModel, typename Column> class AccountRelatedModel : public AccountRelatedModelBase {

  public:
    AccountRelatedModel(utils::libpqxx::ConnectionPool& pool_, const finances::accounts::models::Account& account_,
                        const MovementTypeTableModel* movtype_model_, QObject* parent = nullptr)
        : AccountRelatedModelBase(pool_, account_, movtype_model_, parent) {};

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
            return QString::fromStdString(std::format("{}", items[section].id));
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
        const auto& item = items.at(row);

        switch (role) {
        case Qt::DisplayRole:
            return DataDispatcher<TModel, Column, Qt::DisplayRole>::data(*this, item, column);
        case Qt::FontRole:
            return DataDispatcher<TModel, Column, Qt::FontRole>::data(*this, item, column);
        case Qt::TextAlignmentRole:
            return DataDispatcher<TModel, Column, Qt::TextAlignmentRole>::data(*this, item, column);
        case Qt::BackgroundRole:
            return DataDispatcher<TModel, Column, Qt::BackgroundRole>::data(*this, item, column);
            // default:
            //     break;
        }

        return result;
    }

  protected:
    void _fetch_all() override final {
        SPDLOG_DEBUG("AccountRelatedModel<TModel>::_fetch_all");

        SPDLOG_TRACE(" - fetch all the items for this model");
        typename utils::db::ModelData<TModel>::Manager manager{pool};
        auto all_items = manager.filter_by_fk(account);
        if (!all_items) {
            SPDLOG_ERROR("Error refreshing items");
            // TODO: Communicate error to user
            return;
        }

        SPDLOG_TRACE(" - reset the model");
        this->beginResetModel();
        this->items = std::move(all_items.value());
        this->endResetModel();
    }

  private:
    std::vector<TModel> items;
};

// Overrides for DataDispatcher::data function

template <typename TModel, typename TColumn> struct DataDispatcher<TModel, TColumn, Qt::TextAlignmentRole> {
    static QVariant data(const AccountRelatedModelBase&, const TModel&, TColumn) { return QVariant{Qt::AlignRight}; }
};
