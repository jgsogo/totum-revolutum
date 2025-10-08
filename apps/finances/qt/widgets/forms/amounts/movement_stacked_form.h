#pragma once

#include <QWidget>

#include "apps/finances/qt/metatypes/money.h"

namespace widgets::forms {

    class MovementStackedForm : public QWidget {
        Q_OBJECT

      public:
        explicit MovementStackedForm(QWidget* parent = nullptr);

      signals:
        void amount_changed(QtMoney money);
    };
} // namespace widgets::forms
