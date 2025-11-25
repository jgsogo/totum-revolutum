#include "movement_non_numerable_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"

#include "apps/finances/qt/utils/utils.h"

namespace widgets::forms {

    MovementNonNumerableFormWidget::MovementNonNumerableFormWidget(QWidget* parent) : BaseMovementFormWidget(parent) {
        // Unit value
        unit_value_edit = new MoneyAmountEdit("Amount (%1)", this);
        connect(unit_value_edit, &MoneyAmountEdit::money_changed, [this]() { this->on_input_data_change(); });
        connect(unit_value_edit, &MoneyAmountEdit::money_changed, this,
                &MovementNonNumerableFormWidget::amount_changed);

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(unit_value_edit->get_label(), unit_value_edit);

        this->setLayout(formLayout);
    }

    ExpectedType<finances::accounts::models::Money> MovementNonNumerableFormWidget::getMoneyAmount() const {
        return unit_value_edit->getMoneyAmount();
    }

    void MovementNonNumerableFormWidget::clear() { unit_value_edit->noCcy(); }

    void MovementNonNumerableFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::setCcy(ccy={})", ccy_);

        unit_value_edit->setCcy(ccy_);

        this->on_input_data_change();
    }

    void MovementNonNumerableFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::on_input_data_change()");

        auto money_amount_expected = this->getMoneyAmount();
        if (!money_amount_expected) {
            SPDLOG_WARN("We are skipping this signal: {}", money_amount_expected.error());
            return;
        }

        emit amount_changed(std::move(money_amount_expected.value()));
    }

} // namespace widgets::forms
