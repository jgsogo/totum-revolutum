#include "movement_numerable_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/types/numerable_amount.h"

#include "apps/finances/qt/utils/utils.h"

namespace widgets::forms {

    MovementNumerableFormWidget::MovementNumerableFormWidget(QWidget* parent) : BaseMovementFormWidget(parent) {
        // Quantity
        quantity = new QLineEdit(this);
        connect(quantity, &QLineEdit::textEdited, [this]() { this->on_input_data_change(); });
        quantity->setFocusPolicy(Qt::StrongFocus);

        // Unit value
        unit_value_edit = new MoneyAmountEdit("Unit value (%1)", this);
        connect(unit_value_edit, &MoneyAmountEdit::money_changed,
                [this](finances::accounts::models::Money) { this->on_input_data_change(); });
        unit_value_edit->setFocusPolicy(Qt::StrongFocus);

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(tr("&Quantity"), quantity);
        formLayout->addRow(unit_value_edit->get_label(), unit_value_edit);

        this->setLayout(formLayout);
    }

    ExpectedType<finances::accounts::models::Money> MovementNumerableFormWidget::getMoneyAmount() const {
        auto unit_value_edit_expected = unit_value_edit->getMoneyAmount();
        if (!unit_value_edit_expected) {
            return tl::unexpected{unit_value_edit_expected.error()};
        }

        // FIXME: For quantities, it doesn't make sense the `ccy`!
        auto quantity_expected = utils::qstring_to_amount(quantity->text(), unit_value_edit_expected.value().ccy);
        if (!quantity_expected) {
            return tl::unexpected{quantity_expected.error()};
        }

        finances::investments::models::NumerableAmount amount{.quantity = std::move(quantity_expected.value()),
                                                              .unit_value =
                                                                  std::move(unit_value_edit_expected.value().amount)};
        finances::accounts::models::Money money{amount.amount(), std::move(unit_value_edit_expected.value().ccy)};

        return {std::move(money)};
    }

    ExpectedType<std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                              finances::investments::models::MovementDividend>>
    MovementNumerableFormWidget::populateAdditionalData(finances::accounts::models::Movement&& movement) const {
        SPDLOG_DEBUG("MovementNumerableFormWidget::populateAdditionalData(movement)");
        auto unit_value_edit_expected = unit_value_edit->getMoneyAmount();
        if (!unit_value_edit_expected) {
            return tl::unexpected{unit_value_edit_expected.error()};
        }

        auto quantity_expected = utils::qstring_to_amount(quantity->text(), unit_value_edit_expected.value().ccy);
        if (!quantity_expected) {
            return tl::unexpected{quantity_expected.error()};
        }

        return finances::investments::models::MovementNumerable{
            .movement = std::move(movement),
            .id = {std::monostate{}},
            .quantity = std::move(quantity_expected.value()),
            .unit_value = std::move(unit_value_edit_expected.value()),
        };
    }

    void MovementNumerableFormWidget::clear() {
        this->quantity->clear();
        unit_value_edit->noCcy();
    }

    void MovementNumerableFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementNumerableFormWidget::setCcy(ccy={})", ccy_);
        unit_value_edit->setCcy(ccy_);

        this->on_input_data_change();
    }

    void MovementNumerableFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementNumerableFormWidget::on_input_data_change()");

        auto money_amount_expected = this->getMoneyAmount();
        auto money_opt =
            money_amount_expected.has_value()
                ? std::optional<finances::accounts::models::Money>{std::move(money_amount_expected.value())}
                : std::nullopt;
        emit amount_changed(std::move(money_opt));
    }

} // namespace widgets::forms
