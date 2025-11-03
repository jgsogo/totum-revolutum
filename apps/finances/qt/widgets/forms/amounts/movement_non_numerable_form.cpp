#include "movement_non_numerable_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/utils/utils.h"

namespace widgets::forms {

    MovementNonNumerableFormWidget::MovementNonNumerableFormWidget(QWidget* parent) : QWidget(parent) {
        // We start QLineEdit and disable it, because we don't know the currency!
        amount = new QLineEdit("ccy unknown");
        amount->setEnabled(false);
        connect(amount, &QLineEdit::textEdited, this, [this]() { this->on_input_data_change(); });

        amount_label = new QLabel(tr("Amount"));

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(amount_label, amount);

        this->setLayout(formLayout);
    }

    void MovementNonNumerableFormWidget::clear() {
        ccy = std::nullopt;
        amount->setEnabled(false);
        amount->clear();
    }

    void MovementNonNumerableFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::setCcy(ccy={})", ccy_);
        if (ccy && (ccy.value() == ccy_)) {
            return;
        }
        ccy = ccy_;

        // Create the validator for this currency (different ccys might have different validators)
        QRegularExpression rx(R"(^\d+(,\d{2})?$)");
        QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
        amount->setValidator(ccy_validator);
        amount->setPlaceholderText("120,34");

        // Update label and enable the amount if it was not
        amount_label->setText(tr("Amount (%1)").arg(static_cast<std::string>(ccy.value())));
        amount->setEnabled(true);
        amount->clear();

        this->on_input_data_change();
    }

    void MovementNonNumerableFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::on_input_data_change()");

        if (!ccy) {
            SPDLOG_WARN("There is no currency assigned to this MovementNonNumerableFormWidget! We are skipping this "
                        "notification.");
            return;
        }

        auto amount_expected = utils::qstring_to_amount(amount->text(), ccy.value());
        if (!amount_expected) {
            SPDLOG_ERROR("Invalid amount in QLineEdit field '{}' for ccy {}", amount->text().toStdString(),
                         ccy.value());
            // TODO: Communicate error to the user
            return;
        }

        finances::accounts::models::Money money{std::move(amount_expected.value()), std::move(ccy.value())};
        emit amount_changed(std::move(money));
    }

} // namespace widgets::forms
