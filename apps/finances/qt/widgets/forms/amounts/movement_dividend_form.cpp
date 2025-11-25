#include "movement_dividend_form.h"

#include <spdlog/spdlog.h>

#include <QFormLayout>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "libraries/finances/accounts/cpp/models/types/money.h"
#include "libraries/finances/investments/cpp/models/types/numerable_amount.h"

#include "apps/finances/qt/utils/utils.h"

namespace widgets::forms {

    MovementDividendFormWidget::MovementDividendFormWidget(QWidget* parent) : BaseMovementFormWidget(parent) {
        // Quantity
        ex_dividend_date = new QCalendarWidget();
        connect(ex_dividend_date, &QCalendarWidget::clicked, this,
                &MovementDividendFormWidget::ex_dividend_date_changed);

        // Unit value
        unit_value_edit = new MoneyAmountEdit("Unit value (%1)", this);
        connect(unit_value_edit, &MoneyAmountEdit::money_changed, [this]() { this->on_input_data_change(); });

        unit_value = new QLineEdit("ccy unknown");
        unit_value->setEnabled(false);
        connect(unit_value, &QLineEdit::textEdited, [this]() { this->on_input_data_change(); });

        unit_value_label = new QLabel(tr("Unit value"));

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(tr("Ex dividend &date"), ex_dividend_date);
        formLayout->addRow(unit_value_label, unit_value);
        formLayout->addRow(unit_value_edit->get_label(), unit_value_edit);

        this->setLayout(formLayout);
    }

    ExpectedType<finances::accounts::models::Money> MovementDividendFormWidget::getMoneyAmount() const {
        if (!ccy) {
            return tl::unexpected{error::InputFieldNotSet{"ccy"}};
        }

        if (!quantity) {
            return tl::unexpected{error::InputFieldNotSet{"quantity"}};
        }

        auto unit_value_money_expected = unit_value_edit->getMoneyAmount();
        if (!unit_value_money_expected) {
            return tl::unexpected{error::InputFieldNotSet{"unit_value_edit"}};
        }

        // FIXME: For quantities, it doesn't make sense the `ccy`!
        auto unit_value_expected = utils::qstring_to_amount(unit_value->text(), ccy.value());
        if (!unit_value_expected) {
            return tl::unexpected{unit_value_expected.error()};
        }

        finances::investments::models::NumerableAmount amount{.quantity = std::move(quantity.value()),
                                                              .unit_value = std::move(unit_value_expected.value())};
        finances::accounts::models::Money money{amount.amount(), std::move(ccy.value())};
        return {std::move(money)};
    }

    ExpectedType<std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                              finances::investments::models::MovementDividend>>
    MovementDividendFormWidget::populateAdditionalData(finances::accounts::models::Movement&& movement) const {
        SPDLOG_DEBUG("MovementDividendFormWidget::populateAdditionalData(movement)");

        // - ex_dividend_date
        auto qt_date = ex_dividend_date->selectedDate();
        utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                       date::month{static_cast<unsigned int>(qt_date.month())},
                                                       date::day{static_cast<unsigned int>(qt_date.day())}}};

        // - unit_value
        if (!ccy) {
            return tl::unexpected{error::InputFieldNotSet{"ccy"}};
        }
        auto unit_value_expected = utils::qstring_to_amount(unit_value->text(), ccy.value());
        if (!unit_value_expected) {
            return tl::unexpected{unit_value_expected.error()};
        }

        return finances::investments::models::MovementDividend{
            .movement = std::move(movement),
            .id = {std::monostate{}},
            .ex_dividend_date = std::move(date),
            .unit_value =
                finances::accounts::models::Money{std::move(unit_value_expected.value()), std::move(ccy.value())},
            .snapshot_data = std::nullopt, // TODO
        };
    }

    void MovementDividendFormWidget::clear() {
        ccy = std::nullopt;
        quantity = std::nullopt;
        unit_value->setEnabled(false);
        this->unit_value->clear();
        this->ex_dividend_date->setSelectedDate(QDate::currentDate());
        unit_value_edit->noCcy();
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
        unit_value_label->setText(tr("Unit value (%1)").arg(static_cast<std::string>(ccy.value())));
        unit_value->setEnabled(true);
        unit_value->clear();

        //
        unit_value_edit->setCcy(ccy_);

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

        auto money_amount_expected = this->getMoneyAmount();
        if (!money_amount_expected) {
            SPDLOG_WARN("We are skipping this signal: {}", money_amount_expected.error());
            return;
        }

        emit amount_changed(std::move(money_amount_expected.value()));
    }

} // namespace widgets::forms
