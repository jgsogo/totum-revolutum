#pragma once

#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/types.h"

#include "base_movement_form.h"
#include "money_amount.h"
namespace widgets::forms {

    class MovementNonNumerableFormWidget : public BaseMovementFormWidget {
        Q_OBJECT

      public:
        explicit MovementNonNumerableFormWidget(QWidget* parent = nullptr);

        ExpectedType<finances::accounts::models::Money> getMoneyAmount() const override;

      public slots:
        void clear();

        void setCcy(finances::accounts::models::Ccy);

      private slots:
        void on_input_data_change();

      signals:
        void amount_changed(finances::accounts::models::Money money);

      protected:
        MoneyAmountEdit* unit_value_edit;
    };

} // namespace widgets::forms
