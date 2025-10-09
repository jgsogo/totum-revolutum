#include "movement_dividend_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/types/numerable_amount.h"

#include "apps/finances/qt/metatypes/utils.h"

namespace widgets::forms {

    MovementDividendFormWidget::MovementDividendFormWidget(QWidget* parent) : QWidget(parent) {
        // Quantity
        ex_dividend_date = new QCalendarWidget();
        connect(ex_dividend_date, &QCalendarWidget::clicked, this,
                &MovementDividendFormWidget::ex_dividend_date_changed);

        // Unit value
        unit_value = new QLineEdit("ccy unknown");
        unit_value->setEnabled(false);
        connect(unit_value, &QLineEdit::textChanged, [this]() { this->on_input_data_change(); });

        unit_value_label = new QLabel(tr("&Unit value"));

        QFormLayout* formLayout = new QFormLayout;
        formLayout->addRow(tr("Ex dividend &date"), ex_dividend_date);
        formLayout->addRow(unit_value_label, unit_value);

        this->setLayout(formLayout);
    }

    void MovementDividendFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementDividendFormWidget::setCcy(ccy={})", ccy_);
        if (ccy && (ccy.value() == ccy_)) {
            return;
        }
        ccy = ccy_;

        // Create the validator for this currency (different ccys might have different validators)
        QRegularExpression rx(R"(^\d+(,\d{2})?$)");
        QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
        unit_value->setValidator(ccy_validator);
        unit_value->setPlaceholderText("120,34");

        // Update label and enable the inputs
        unit_value_label->setText(tr("&Unit value (%1)").arg(static_cast<std::string>(ccy.value())));
        unit_value->setEnabled(true);

        this->on_input_data_change();
    }

    void MovementDividendFormWidget::setQuantity(finances::accounts::models::Amount quantity_) {
        SPDLOG_DEBUG("MovementDividendFormWidget::setQuantity(quantity={})", quantity_);
        if (quantity && (quantity.value() == quantity_)) {
            return;
        }
        quantity = quantity_;

        this->on_input_data_change();
    }

    void MovementDividendFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementDividendFormWidget::on_input_data_change()");

        if (!ccy) {
            SPDLOG_WARN("There is no currency assigned to this MovementDividendFormWidget! We are skipping this "
                        "notification.");
            return;
        }

        if (!quantity) {
            SPDLOG_WARN("There is no quantity assigned to this MovementDividendFormWidget! We are skipping this "
                        "notification.");
            return;
        }

        // FIXME: For quantities, it doesn't make sense the `ccy`!
        auto unit_value_expected = utils::qstring_to_amount(unit_value->text(), ccy.value());
        if (!unit_value_expected) {
            SPDLOG_ERROR("Invalid unit_value in QLineEdit field '{}'", unit_value->text().toStdString());
            // TODO: Communicate error to the user
            return;
        }

        finances::investments::models::NumerableAmount amount{.quantity = quantity.value(),
                                                              .unit_value = unit_value_expected.value()};
        finances::accounts::models::Money money{amount.amount(), ccy.value()};
        emit amount_changed(QtMoney{money});
    }

} // namespace widgets::forms
