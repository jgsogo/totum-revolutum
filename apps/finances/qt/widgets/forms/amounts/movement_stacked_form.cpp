#include "movement_stacked_form.h"

#include <QHBoxLayout>
#include <QPushButton>
#include <QStackedLayout>
#include <QVBoxLayout>

#include "movement_dividend_form.h"
#include "movement_non_numerable_form.h"
#include "movement_numerable_form.h"

namespace widgets::forms {

    struct MovementStackedForm::Impl {
        MovementNonNumerableFormWidget* mov_non_numerable;
        MovementNumerableFormWidget* mov_numerable;
        MovementDividendFormWidget* mov_dividend;
        QStackedLayout* stacked_layout;

        QPushButton* non_numerable;
        QPushButton* numerable;
        QPushButton* dividend;
    };

    MovementStackedForm::MovementStackedForm(QWidget* parent)
        : QWidget(parent), pImpl{std::make_unique<MovementStackedForm::Impl>()} {

        pImpl->stacked_layout = new QStackedLayout;

        {
            pImpl->mov_non_numerable = new MovementNonNumerableFormWidget;
            connect(pImpl->mov_non_numerable, &MovementNonNumerableFormWidget::amount_changed, this,
                    &MovementStackedForm::amount_changed);
            pImpl->mov_non_numerable->blockSignals(true);
            pImpl->stacked_layout->addWidget(pImpl->mov_non_numerable);
        }
        {
            pImpl->mov_numerable = new MovementNumerableFormWidget;
            connect(pImpl->mov_numerable, &MovementNumerableFormWidget::amount_changed, this,
                    &MovementStackedForm::amount_changed);
            pImpl->mov_numerable->blockSignals(true);
            pImpl->stacked_layout->addWidget(pImpl->mov_numerable);
        }
        {
            pImpl->mov_dividend = new MovementDividendFormWidget;
            connect(pImpl->mov_dividend, &MovementDividendFormWidget::amount_changed, this,
                    &MovementStackedForm::amount_changed);
            connect(pImpl->mov_dividend, &MovementDividendFormWidget::ex_dividend_date_changed, this,
                    &MovementStackedForm::ex_dividend_date_changed);
            pImpl->mov_dividend->blockSignals(true);
            pImpl->stacked_layout->addWidget(pImpl->mov_dividend);
        }

        // Buttons
        QHBoxLayout* buttons = new QHBoxLayout();
        {
            pImpl->non_numerable = new QPushButton(tr("Non numerable"), this);
            pImpl->numerable = new QPushButton(tr("Numerable"), this);
            pImpl->dividend = new QPushButton(tr("Dividend"), this);

            connect(pImpl->non_numerable, &QPushButton::clicked, this,
                    &MovementStackedForm::set_movement_non_numerable);
            connect(pImpl->numerable, &QPushButton::clicked, this, &MovementStackedForm::set_movement_numerable);
            connect(pImpl->dividend, &QPushButton::clicked, this, &MovementStackedForm::set_movement_dividend);

            buttons->addWidget(pImpl->non_numerable);
            buttons->addWidget(pImpl->numerable);
            buttons->addWidget(pImpl->dividend);
        }

        // Layout
        QVBoxLayout* mainLayout = new QVBoxLayout();
        mainLayout->addLayout(buttons);
        mainLayout->addLayout(pImpl->stacked_layout);

        this->setLayout(mainLayout);

        connect(pImpl->stacked_layout, &QStackedLayout::currentChanged, this, &MovementStackedForm::on_current_changed);
    }

    MovementStackedForm::~MovementStackedForm() = default;

    ExpectedType<finances::accounts::models::Money> MovementStackedForm::getMoneyAmount() const {
        BaseMovementFormWidget* cur_widget =
            static_cast<BaseMovementFormWidget*>(pImpl->stacked_layout->currentWidget());
        return cur_widget->getMoneyAmount();
    }

    ExpectedType<std::variant<finances::accounts::models::Movement, finances::investments::models::MovementNumerable,
                              finances::investments::models::MovementDividend>>
    MovementStackedForm::populateAdditionalData(finances::accounts::models::Movement&& movement) const {
        BaseMovementFormWidget* cur_widget =
            static_cast<BaseMovementFormWidget*>(pImpl->stacked_layout->currentWidget());
        return cur_widget->populateAdditionalData(std::move(movement));
    }

    void MovementStackedForm::clear() {
        pImpl->mov_non_numerable->clear();
        pImpl->mov_numerable->clear();
        pImpl->mov_dividend->clear();

        this->set_movement_non_numerable();
    }

    void MovementStackedForm::show_buttons() {
        pImpl->non_numerable->setVisible(true);
        pImpl->numerable->setVisible(true);
        pImpl->dividend->setVisible(true);
    }

    void MovementStackedForm::hide_buttons() {
        pImpl->non_numerable->setVisible(false);
        pImpl->numerable->setVisible(false);
        pImpl->dividend->setVisible(false);
    }

    void MovementStackedForm::set_movement_non_numerable() {
        pImpl->mov_non_numerable->blockSignals(false);
        pImpl->mov_numerable->blockSignals(true);
        pImpl->mov_dividend->blockSignals(true);

        pImpl->non_numerable->setDown(true);
        pImpl->numerable->setDown(false);
        pImpl->dividend->setDown(false);

        pImpl->stacked_layout->setCurrentWidget(pImpl->mov_non_numerable);
    }

    void MovementStackedForm::set_movement_numerable() {
        pImpl->mov_non_numerable->blockSignals(true);
        pImpl->mov_numerable->blockSignals(false);
        pImpl->mov_dividend->blockSignals(true);

        pImpl->non_numerable->setDown(false);
        pImpl->numerable->setDown(true);
        pImpl->dividend->setDown(false);

        pImpl->stacked_layout->setCurrentWidget(pImpl->mov_numerable);
    }

    void MovementStackedForm::set_movement_dividend() {
        pImpl->mov_non_numerable->blockSignals(true);
        pImpl->mov_numerable->blockSignals(true);
        pImpl->mov_dividend->blockSignals(false);

        pImpl->non_numerable->setDown(false);
        pImpl->numerable->setDown(false);
        pImpl->dividend->setDown(true);

        pImpl->stacked_layout->setCurrentWidget(pImpl->mov_dividend);
    }

    void MovementStackedForm::set_ccy(finances::accounts::models::Ccy ccy) {
        pImpl->mov_non_numerable->setCcy(ccy);
        pImpl->mov_numerable->setCcy(ccy);
        pImpl->mov_dividend->setCcy(ccy);
    }

    void MovementStackedForm::set_dividend_quantity(finances::accounts::models::Amount quantity) {
        pImpl->mov_dividend->setQuantity(quantity);
    }

    void MovementStackedForm::on_current_changed(int index) {
        auto money_expected = this->getMoneyAmount();
        auto money = money_expected.has_value()
                         ? std::optional<finances::accounts::models::Money>{std::move(money_expected.value())}
                         : std::nullopt;
        emit amount_changed(std::move(money));
    }

} // namespace widgets::forms
