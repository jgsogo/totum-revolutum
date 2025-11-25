#pragma once

#include <QCalendarWidget>
#include <QLabel>
#include <QLineEdit>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/types.h"

#include "base_movement_form.h"
#include "money_amount.h"
namespace widgets::forms {

    class MovementDividendFormWidget : public BaseMovementFormWidget {
        Q_OBJECT

      public:
        explicit MovementDividendFormWidget(QWidget* parent = nullptr);

        ExpectedType<finances::accounts::models::Money> getMoneyAmount() const override;

        ExpectedType<
            std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                         finances::investments::models::MovementDividend>>
        populateAdditionalData(finances::accounts::models::Movement&& movement) const override;

      public slots:
        void clear();

        void setCcy(finances::accounts::models::Ccy);
        void setQuantity(finances::accounts::models::Amount);

      private slots:
        void on_input_data_change();

      signals:
        void ex_dividend_date_changed(QDate date);
        void amount_changed(std::optional<finances::accounts::models::Money>);

      protected:
        std::optional<finances::accounts::models::Amount> quantity;

        QCalendarWidget* ex_dividend_date;
        MoneyAmountEdit* unit_value_edit;
    };

} // namespace widgets::forms
