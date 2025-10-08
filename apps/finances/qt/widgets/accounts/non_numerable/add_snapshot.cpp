#include "add_snapshot.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

#include "libraries/finances/accounts/cpp/models/snapshot.h"

#include "apps/finances/qt/metatypes/utils.h"

AddSnapshotNonNumerableWidget::AddSnapshotNonNumerableWidget(utils::libpqxx::ConnectionPool& pool_,
                                                             const finances::accounts::models::Account& account_,
                                                             QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_}, account{account_} {
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
    QString title(tr("%1 - Add snapshot non numerable").arg(account.name));
    mainLayout->addWidget(new QLabel(title));
    mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
}

void AddSnapshotNonNumerableWidget::add_snapshot_clicked() {
    SPDLOG_DEBUG("AddSnapshotNonNumerableWidget::add_snapshot_clicked");

    if (!amount->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    auto qt_date = calendar->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    auto amount_amount = utils::qstring_to_amount(amount->text(), account.ccy);
    if (!amount_amount) {
        // TODO: Communicate error to the user
        return;
    }

    finances::accounts::models::SnapshotManager manager{pool};
    auto r = manager.create(account.id, std::move(date), std::move(amount_amount.value()));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account");
        // TODO: Communicate error to user
        return;
    }

    emit new_snapshot(account.id);

    this->accept();
}
