#include "account_add_snapshot_numerable.h"

#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

AddSnapshotNumerableWidget::AddSnapshotNumerableWidget(QWidget* parent, Qt::WindowFlags f) : QDialog(parent, f) {
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);

    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* amount_validator = new QRegularExpressionValidator(rx, this);

    unit_value = new QLineEdit(this);
    unit_value->setValidator(amount_validator);
    unit_value->setPlaceholderText("120,34");

    quantity = new QLineEdit(this);
    quantity->setValidator(amount_validator);
    quantity->setPlaceholderText("120,34");

    QPushButton* add = new QPushButton(tr("Add"), this);
    connect(add, &QPushButton::clicked, this, &AddSnapshotNumerableWidget::add_snapshot_clicked);

    QPushButton* cancel = new QPushButton(tr("Cancel"), this);
    connect(cancel, &QPushButton::clicked, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(calendar);
    mainLayout->addWidget(quantity);
    mainLayout->addWidget(unit_value);
    mainLayout->addWidget(add);
    mainLayout->addWidget(cancel);

    this->setLayout(mainLayout);
    this->setWindowTitle(tr("Add snapshot numerable"));
}

void AddSnapshotNumerableWidget::add_snapshot_clicked() {
    SPDLOG_DEBUG("AddSnapshotNumerableWidget::add_snapshot_clicked");

    if (!unit_value->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    if (!quantity->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    auto date_ = calendar->selectedDate();
    auto unit_value_ = unit_value->text();
    auto quantity_ = quantity->text();
    auto snapshot = SnapshotNumerable::from(std::move(date_), std::move(quantity_), std::move(unit_value_));
    if (!snapshot) {
        SPDLOG_WARN("Failed to create snapshot from date={}, quantity={} and unit_value={}: {}",
                    date_.toString().toStdString(), quantity_.toStdString(), unit_value_.toStdString(),
                    snapshot.error());
        // TODO: Communicate error to the user
        return;
    }

    emit new_snapshot(snapshot.value());

    this->accept();
}
