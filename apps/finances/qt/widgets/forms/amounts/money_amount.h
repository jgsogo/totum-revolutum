#pragma once

#include <QLabel>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"
#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/errors/errors.hpp"

class MoneyAmountEdit : public QLineEdit {
    Q_OBJECT
  public:
    explicit MoneyAmountEdit(QString label_template, QWidget* parent = nullptr);
    ~MoneyAmountEdit();

    const QLabel& get_label() const;
    ExpectedType<finances::accounts::models::Money> getMoneyAmount() const;

  public slots:
    void setCcy(finances::accounts::models::Ccy);
    void noCcy();

  private slots:
    void on_text_edited(const QString& text);

  signals:
    void money_changed(finances::accounts::models::Money);

  private:
    struct Impl;
    std::unique_ptr<Impl> pImpl;
};
