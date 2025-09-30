#include "add_snapshot.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

AddSnapshotNonNumerableWidget::AddSnapshotNonNumerableWidget(const finances::accounts::models::Account& account_,
                                                             QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), account{account_} {
    // Components
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);

    amount = new QLineEdit(this);
    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
    amount->setValidator(ccy_validator);
    amount->setPlaceholderText("120,34");

    // Layout
    QFormLayout* formLayout = new QFormLayout;
    formLayout->addRow(tr("&Date:"), calendar);
    formLayout->addRow(tr("&Amount (%1):").arg(static_cast<std::string>(account.ccy)), amount);

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddSnapshotNonNumerableWidget::add_snapshot_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout;
    mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

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
