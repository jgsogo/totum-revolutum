#pragma once

#include <QWidget>

#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/movement_dividend.h"
#include "libraries/finances/investments/cpp/models/movement_numerable.h"

#include "apps/finances/qt/errors/errors.hpp"

namespace widgets::forms {

    class BaseMovementFormWidget : public QWidget {
      public:
        explicit BaseMovementFormWidget(QWidget* parent = nullptr) : QWidget{parent} {};

        virtual ExpectedType<finances::accounts::models::Money> getMoneyAmount() const = 0;

        virtual ExpectedType<
            std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                         finances::investments::models::MovementDividend>>
        populateAdditionalData(finances::accounts::models::Movement&& movement) const {
            return std::move(movement);
        }
    };

} // namespace widgets::forms
