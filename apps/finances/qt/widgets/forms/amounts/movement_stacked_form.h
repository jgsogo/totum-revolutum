#pragma once

#include <QDate>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/investments/cpp/models/movement_dividend.h"
#include "libraries/finances/investments/cpp/models/movement_numerable.h"

#include "apps/finances/qt/errors/errors.hpp"
#include "apps/finances/qt/metatypes/types.h"

namespace widgets::forms {

    class MovementStackedForm : public QWidget {
        Q_OBJECT

      public:
        explicit MovementStackedForm(QWidget* parent = nullptr);
        ~MovementStackedForm();

        ExpectedType<finances::accounts::models::Money> getMoneyAmount() const;

        ExpectedType<
            std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                         finances::investments::models::MovementDividend>>
        populateAdditionalData(finances::accounts::models::Movement&& movement) const;

      public slots:
        void clear();

        void show_buttons();
        void hide_buttons();

        void set_movement_non_numerable();
        void set_movement_numerable();
        void set_movement_dividend();

        void set_ccy(finances::accounts::models::Ccy ccy);
        void set_dividend_quantity(finances::accounts::models::Amount quantity);

      signals:
        void amount_changed(finances::accounts::models::Money money);
        void ex_dividend_date_changed(QDate date);

      private:
        struct Impl;
        std::unique_ptr<Impl> pImpl;
    };
} // namespace widgets::forms
