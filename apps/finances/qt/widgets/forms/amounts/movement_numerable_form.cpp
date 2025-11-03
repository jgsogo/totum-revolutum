#include "movement_numerable_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/types/numerable_amount.h"

#include "apps/finances/qt/utils/utils.h"

namespace widgets::forms {

    MovementNumerableFormWidget::MovementNumerableFormWidget(QWidget* parent) : QWidget(parent) {
        // Quantity
        quantity = new QLineEdit();
        connect(quantity, &QLineEdit::textEdited, [this]() { this->on_input_data_change(); });

        QRegularExpression rx(R"(^\d+(,\d{2})?$)");
        QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
        quantity->setValidator(ccy_validator);
        quantity->setPlaceholderText("120,34");

        // Unit value
        unit_value = new QLineEdit("ccy unknown");
        unit_value->setEnabled(false);
        connect(unit_value, &QLineEdit::textEdited, [this]() { this->on_input_data_change(); });

        unit_value_label = new QLabel(tr("&Unit value"));

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(tr("&Quantity"), quantity);
        formLayout->addRow(unit_value_label, unit_value);

        this->setLayout(formLayout);
    }

    void MovementNumerableFormWidget::clear() {
        ccy = std::nullopt;
        unit_value->setEnabled(false);

        this->quantity->clear();
        this->unit_value->clear();
    }

    void MovementNumerableFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementNumerableFormWidget::setCcy(ccy={})", ccy_);
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
        unit_value_label->setText(tr("Unit value (%1)").arg(static_cast<std::string>(ccy.value())));

        unit_value->setEnabled(true);
        unit_value->clear();

        this->on_input_data_change();
    }

    void MovementNumerableFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementNumerableFormWidget::on_input_data_change()");

        if (!ccy) {
            SPDLOG_WARN("There is no currency assigned to this MovementNumerableFormWidget! We are skipping this "
                        "notification.");
            return;
        }

        // FIXME: For quantities, it doesn't make sense the `ccy`!
        auto quantity_expected = utils::qstring_to_amount(quantity->text(), ccy.value());
        if (!quantity_expected) {
            SPDLOG_ERROR("Invalid quantity in QLineEdit field '{}'", quantity->text().toStdString());
            // TODO: Communicate error to the user
            return;
        }

        auto unit_value_expected = utils::qstring_to_amount(unit_value->text(), ccy.value());
        if (!unit_value_expected) {
            SPDLOG_ERROR("Invalid unit_value in QLineEdit field '{}'", unit_value->text().toStdString());
            // TODO: Communicate error to the user
            return;
        }

        finances::investments::models::NumerableAmount amount{.quantity = std::move(quantity_expected.value()),
                                                              .unit_value = std::move(unit_value_expected.value())};
        finances::accounts::models::Money money{amount.amount(), std::move(ccy.value())};
        emit amount_changed(std::move(money));
    }

} // namespace widgets::forms
