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
        unit_value_edit->setFocusPolicy(Qt::StrongFocus);

        // Quantity
        quantity_label = new QLabel("<no sanpshot>");

        QFormLayout* formLayout = new QFormLayout;
        formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
        formLayout->addRow(tr("Ex dividend &date"), ex_dividend_date);
        formLayout->addRow(tr("Quantity"), quantity_label);
        formLayout->addRow(unit_value_edit->get_label(), unit_value_edit);

        this->setLayout(formLayout);
    }

    ExpectedType<finances::accounts::models::Money> MovementDividendFormWidget::getMoneyAmount() const {
        if (!closest_snapshot) {
            return tl::unexpected{error::InputFieldNotSet{"quantity"}};
        }

        auto unit_value_money_expected = unit_value_edit->getMoneyAmount();
        if (!unit_value_money_expected) {
            return tl::unexpected{error::InputFieldNotSet{"unit_value_edit"}};
        }

        finances::investments::models::NumerableAmount amount{.quantity = std::move(closest_snapshot.value().quantity),
                                                              .unit_value =
                                                                  std::move(unit_value_money_expected.value().amount)};
        finances::accounts::models::Money money{amount.amount(), std::move(unit_value_money_expected.value().ccy)};
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
        auto unit_value_money_expected = unit_value_edit->getMoneyAmount();
        if (!unit_value_money_expected) {
            return tl::unexpected{error::InputFieldNotSet{"unit_value_edit"}};
        }

        return finances::investments::models::MovementDividend{
            .movement = std::move(movement),
            .id = {std::monostate{}},
            .ex_dividend_date = std::move(date),
            .unit_value = unit_value_money_expected.value(),
            .snapshot_data = std::nullopt, // TODO
        };
    }

    void MovementDividendFormWidget::clear() {
        closest_snapshot = std::nullopt;
        this->ex_dividend_date->setSelectedDate(QDate::currentDate());
        unit_value_edit->noCcy();
    }

    void MovementDividendFormWidget::setCcy(finances::accounts::models::Ccy ccy_) {
        SPDLOG_DEBUG("MovementDividendFormWidget::setCcy(ccy={})", ccy_);
        //
        unit_value_edit->setCcy(ccy_);

        this->on_input_data_change();
    }

    void MovementDividendFormWidget::setClosestSnapshot(
        std::optional<finances::investments::models::SnapshotNumerable> snapshot) {
        SPDLOG_DEBUG("MovementDividendFormWidget::setClosestSnapshot(snapshot)");
        closest_snapshot = std::move(snapshot);

        if (closest_snapshot) {
            quantity_label->setText(
                QString("%1 (@ %2)")
                    .arg(static_cast<std::string>(closest_snapshot.value().quantity),
                         utils::date_to_qdate(closest_snapshot.value().snapshot.date_value).toString()));
        } else {
            quantity_label->setText("<no snapshot>");
        }

        this->on_input_data_change();
    }

    void MovementDividendFormWidget::on_input_data_change() {
        SPDLOG_DEBUG("MovementDividendFormWidget::on_input_data_change()");

        auto money_amount_expected = this->getMoneyAmount();
        auto money_opt =
            money_amount_expected.has_value()
                ? std::optional<finances::accounts::models::Money>{std::move(money_amount_expected.value())}
                : std::nullopt;
        emit amount_changed(std::move(money_opt));
    }

} // namespace widgets::forms
