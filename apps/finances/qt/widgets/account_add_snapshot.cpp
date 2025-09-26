#include "account_add_snapshot.h"

#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

Snapshot::Snapshot(const QDate& date, const QString& amount) : date_{date}, amount_{amount} {}

QDate Snapshot::date() const { return date_; }

QStringView Snapshot::amount() const { return amount_; }

AddSnapshotWidget::AddSnapshotWidget(QWidget* parent, Qt::WindowFlags f) : QDialog(parent, f) {
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);

    amount = new QLineEdit(this);
    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* ccy_validator = new QRegularExpressionValidator(rx, this);
    amount->setValidator(ccy_validator);
    amount->setPlaceholderText("120,34");

    QPushButton* add = new QPushButton(tr("Add"), this);
    connect(add, &QPushButton::clicked, this, &AddSnapshotWidget::add_snapshot);

    QPushButton* cancel = new QPushButton(tr("Cancel"), this);
    connect(cancel, &QPushButton::clicked, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout();
    mainLayout->addWidget(calendar);
    mainLayout->addWidget(amount);
    mainLayout->addWidget(add);
    mainLayout->addWidget(cancel);

    this->setLayout(mainLayout);
    this->setWindowTitle(tr("Add snapshot"));
}

void AddSnapshotWidget::add_snapshot() {
    SPDLOG_DEBUG("AddSnapshotWidget::add_snapshot");

    if (!amount->hasAcceptableInput()) {
        return;
    }

    auto date_ = calendar->selectedDate();
    auto amount_ = amount->text();
    Snapshot snapshot{date_, amount_};
    emit new_snapshot(snapshot);

    this->accept();
}
