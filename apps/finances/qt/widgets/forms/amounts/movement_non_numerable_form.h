#pragma once

#include <QLabel>
#include <QLineEdit>
#include <QWidget>

#include "libraries/finances/accounts/cpp/models/types/ccy.h"

#include "apps/finances/qt/metatypes/types.h"

namespace widgets::forms {

    class MovementNonNumerableFormWidget : public QWidget {
        Q_OBJECT

      public:
        explicit MovementNonNumerableFormWidget(QWidget* parent = nullptr);

      public slots:
        void clear();

        void setCcy(finances::accounts::models::Ccy);

      private slots:
        void on_input_data_change();

      signals:
        void amount_changed(finances::accounts::models::Money money);

      protected:
        std::optional<finances::accounts::models::Ccy> ccy;
        QLineEdit* amount;
        QLabel* amount_label;
    };

} // namespace widgets::forms
