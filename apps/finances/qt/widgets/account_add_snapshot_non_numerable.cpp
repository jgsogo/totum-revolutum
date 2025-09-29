#include "account_add_snapshot_non_numerable.h"

#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

AddSnapshotNonNumerableWidget::AddSnapshotNonNumerableWidget(QWidget* parent, Qt::WindowFlags f) : QDialog(parent, f) {
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);

    amount = new QLineEdit(this);
    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
    amount->setValidator(ccy_validator);
    amount->setPlaceholderText("120,34");

    QPushButton* add = new QPushButton(tr("Add"), this);
    connect(add, &QPushButton::clicked, this, &AddSnapshotNonNumerableWidget::add_snapshot_clicked);

    QPushButton* cancel = new QPushButton(tr("Cancel"), this);
    connect(cancel, &QPushButton::clicked, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(calendar);
    mainLayout->addWidget(amount);
    mainLayout->addWidget(add);
    mainLayout->addWidget(cancel);

    this->setLayout(mainLayout);
    this->setWindowTitle(tr("Add snapshot non numerable"));
}

void AddSnapshotNonNumerableWidget::add_snapshot_clicked() {
    SPDLOG_DEBUG("AddSnapshotNonNumerableWidget::add_snapshot_clicked");

    if (!amount->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    auto date_ = calendar->selectedDate();
    auto amount_ = amount->text();
    auto snapshot = SnapshotNonNumerable::from(std::move(date_), std::move(amount_));
    if (!snapshot) {
        SPDLOG_WARN("Failed to create snapshot from date={} and amount={}: {}", date_.toString().toStdString(),
                    amount_.toStdString(), snapshot.error());
        // TODO: Communicate error to the user
        return;
    }

    emit new_snapshot(snapshot.value());

    this->accept();
}
