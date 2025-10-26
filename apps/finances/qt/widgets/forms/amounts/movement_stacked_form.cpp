#include "movement_stacked_form.h"

#include <QStackedLayout>

#include "movement_dividend_form.h"
#include "movement_non_numerable_form.h"
#include "movement_numerable_form.h"

namespace widgets::forms {

    struct MovementStackedForm::Impl {
        MovementNonNumerableFormWidget* mov_non_numerable;
        MovementNumerableFormWidget* mov_numerable;
        MovementDividendFormWidget* mov_dividend;
        QStackedLayout* stacked_layout;
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

        this->setLayout(pImpl->stacked_layout);
    }

    MovementStackedForm::~MovementStackedForm() = default;

    void MovementStackedForm::set_movement_non_numerable() {
        pImpl->mov_non_numerable->blockSignals(false);
        pImpl->mov_numerable->blockSignals(true);
        pImpl->mov_dividend->blockSignals(true);

        pImpl->stacked_layout->setCurrentWidget(pImpl->mov_non_numerable);
    }

    void MovementStackedForm::set_movement_numerable() {
        pImpl->mov_non_numerable->blockSignals(true);
        pImpl->mov_numerable->blockSignals(false);
        pImpl->mov_dividend->blockSignals(true);

        pImpl->stacked_layout->setCurrentWidget(pImpl->mov_numerable);
    }

    void MovementStackedForm::set_movement_dividend() {
        pImpl->mov_non_numerable->blockSignals(true);
        pImpl->mov_numerable->blockSignals(true);
        pImpl->mov_dividend->blockSignals(false);

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

} // namespace widgets::forms
