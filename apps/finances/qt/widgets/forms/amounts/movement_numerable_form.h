#pragma once

#include <QLabel>
#include <QLineEdit>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/money.h"

namespace widgets::forms {

    class MovementNumerableFormWidget : public QWidget {
        Q_OBJECT

      public:
        explicit MovementNumerableFormWidget(QWidget* parent = nullptr);

      public slots:
        void setCcy(finances::accounts::models::Ccy);

      private slots:
        void on_input_data_change();

      signals:
        void amount_changed(QtMoney money);

      protected:
        std::optional<finances::accounts::models::Ccy> ccy;
        QLineEdit* quantity;
        QLineEdit* unit_value;
        QLabel* unit_value_label;
    };

} // namespace widgets::forms
