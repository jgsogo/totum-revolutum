#pragma once

#include <QDialog>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "libraries/finances/accounts/cpp/models/transaction.h"

class TransactionDetailWidget : public QDialog {
    Q_OBJECT
  public:
    explicit TransactionDetailWidget(utils::libpqxx::ConnectionPool& pool,
                                     const finances::accounts::models::Transaction&, QWidget* parent = nullptr,
                                     Qt::WindowFlags f = Qt::WindowFlags());

  protected:
    utils::libpqxx::ConnectionPool& pool;
    const finances::accounts::models::Transaction& transaction;
};
