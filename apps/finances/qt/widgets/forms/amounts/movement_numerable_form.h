#pragma once

#include <QLineEdit>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/types.h"

#include "base_movement_form.h"
#include "money_amount.h"

namespace widgets::forms {

    class MovementNumerableFormWidget : public BaseMovementFormWidget {
        Q_OBJECT

      public:
        explicit MovementNumerableFormWidget(QWidget* parent = nullptr);

        ExpectedType<finances::accounts::models::Money> getMoneyAmount() const override;

        ExpectedType<
            std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                         finances::investments::models::MovementDividend>>
        populateAdditionalData(finances::accounts::models::Movement&& movement) const override;

      public slots:
        void clear();

        void setCcy(finances::accounts::models::Ccy);

      private slots:
        void on_input_data_change();

      signals:
        void amount_changed(std::optional<finances::accounts::models::Money>);

      protected:
        QLineEdit* quantity;
        MoneyAmountEdit* unit_value_edit;
    };

} // namespace widgets::forms
