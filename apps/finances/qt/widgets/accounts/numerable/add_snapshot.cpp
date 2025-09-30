#include "add_snapshot.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

AddSnapshotNumerableWidget::AddSnapshotNumerableWidget(const finances::accounts::models::Account& account_,
                                                       QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), account{account_} {
    // components
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);

    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* amount_validator = new QRegularExpressionValidator(rx, this);

    quantity = new QLineEdit(this);
    quantity->setValidator(amount_validator);
    quantity->setPlaceholderText("120,34");

    unit_value = new QLineEdit(this);
    unit_value->setValidator(amount_validator);
    unit_value->setPlaceholderText("120,34");

    // Layout
    QFormLayout* formLayout = new QFormLayout;
    formLayout->addRow(tr("&Date:"), calendar);
    formLayout->addRow(tr("&Quantity:"), quantity);
    formLayout->addRow(tr("&Unit value (%1):").arg(static_cast<std::string>(account.ccy)), unit_value);

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddSnapshotNumerableWidget::add_snapshot_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout;
    mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

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
