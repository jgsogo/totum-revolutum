#pragma once

#include <QLabel>
#include <QLineEdit>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/types.h"

#include "base_movement_form.h"

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
        void amount_changed(finances::accounts::models::Money money);

      protected:
        std::optional<finances::accounts::models::Ccy> ccy;
        QLineEdit* quantity;
        QLineEdit* unit_value;
        QLabel* unit_value_label;
    };

} // namespace widgets::forms
