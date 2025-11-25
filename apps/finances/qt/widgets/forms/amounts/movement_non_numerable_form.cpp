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
        unit_value_edit->setFocusPolicy(Qt::StrongFocus);

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(unit_value_edit->get_label(), unit_value_edit);

        this->setLayout(formLayout);
    }

    ExpectedType<finances::accounts::models::Money> MovementNonNumerableFormWidget::getMoneyAmount() const {
        return unit_value_edit->getMoneyAmount();
    }

    void MovementNonNumerableFormWidget::clear() {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::clear()");
        unit_value_edit->noCcy();
    }

    void MovementNonNumerableFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::setCcy(ccy={})", ccy_);
        unit_value_edit->setCcy(ccy_);
    }

    void MovementNonNumerableFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementNonNumerableFormWidget::on_input_data_change()");

        auto money_amount_expected = this->getMoneyAmount();
        auto money_opt =
            money_amount_expected.has_value()
                ? std::optional<finances::accounts::models::Money>{std::move(money_amount_expected.value())}
                : std::nullopt;
        emit amount_changed(std::move(money_opt));
    }

} // namespace widgets::forms
