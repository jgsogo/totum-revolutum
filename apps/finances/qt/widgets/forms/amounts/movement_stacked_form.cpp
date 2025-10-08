#include "movement_stacked_form.h"

#include <QStackedLayout>

#include "movement_dividend_form.h"
#include "movement_non_numerable_form.h"
#include "movement_numerable_form.h"

namespace widgets::forms {

    MovementStackedForm::MovementStackedForm(QWidget* parent) : QWidget(parent) {

        MovementNonNumerableFormWidget* mov_non_numerable = new MovementNonNumerableFormWidget;
        connect(mov_non_numerable, &MovementNonNumerableFormWidget::amount_changed, this,
                &MovementStackedForm::amount_changed);

        MovementNumerableFormWidget* mov_numerable = new MovementNumerableFormWidget;
        connect(mov_numerable, &MovementNumerableFormWidget::amount_changed, this,
                &MovementStackedForm::amount_changed);

        MovementDividendFormWidget* mov_dividend = new MovementDividendFormWidget;
        connect(mov_dividend, &MovementDividendFormWidget::amount_changed, this, &MovementStackedForm::amount_changed);

        QStackedLayout* stacked_layout = new QStackedLayout;
        stacked_layout->addWidget(mov_non_numerable);
        stacked_layout->addWidget(mov_numerable);
        stacked_layout->addWidget(mov_dividend);

        this->setLayout(stacked_layout);
    }
} // namespace widgets::forms
