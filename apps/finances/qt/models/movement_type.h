#pragma once

#include <QAbstractTableModel>

#include "libraries/finances/accounts/cpp/models/hierarchy_tree.h"

class MovementTypeTableModel : public QAbstractTableModel {
    Q_OBJECT
  public:
    enum class Column {
        ID = 0,
        NAME = 2,
        BREADCRUMB = 3,
    };

  public:
    MovementTypeTableModel(utils::libpqxx::ConnectionPool& pool, QObject* parent = nullptr);

    int rowCount(const QModelIndex& parent = QModelIndex()) const override;
    int columnCount(const QModelIndex& parent = QModelIndex()) const override;
    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    const finances::accounts::models::MovementType& get_movement_type(finances::accounts::models::Id) const;
    std::optional<std::reference_wrapper<
        const std::vector<std::pair<decltype(finances::accounts::models::MovementType::id), std::string>>>>
        get_breadcrumb(finances::accounts::models::Id) const;

  private slots:
    void fetch_all();
    void fetch_breadcrumbs();

  private:
    utils::libpqxx::ConnectionPool& pool;
    std::vector<finances::accounts::models::MovementType> items;
    std::map<finances::accounts::models::Id,
             std::vector<std::pair<decltype(finances::accounts::models::MovementType::id), std::string>>>
        breadcrumbs;
};
