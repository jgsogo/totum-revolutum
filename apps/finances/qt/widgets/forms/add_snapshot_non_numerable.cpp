#include "add_snapshot_non_numerable.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>

#include <spdlog/spdlog.h>

#include "libraries/finances/accounts/cpp/models/snapshot.h"

#include "apps/finances/qt/utils/utils.h"

AddSnapshotNonNumerableWidget::AddSnapshotNonNumerableWidget(utils::libpqxx::ConnectionPool& pool_,
                                                             const finances::accounts::models::Account& account_,
                                                             QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_}, account{account_} {
    // Components
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);
    calendar->setFocusPolicy(Qt::StrongFocus);

    amount = new MoneyAmountEdit("Amount (%1)", this);
    amount->setCcy(account.ccy);
    amount->setFocusPolicy(Qt::StrongFocus);

    // Layout
    QFormLayout* formLayout = new QFormLayout;
    formLayout->addRow(tr("&Date:"), calendar);
    formLayout->addRow(amount->get_label(), amount);

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Save | QDialogButtonBox::Cancel);
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

    auto money_expected = amount->getMoneyAmount();
    if (!money_expected) {
        // TODO: Communicate error to the user
        return;
    }

    auto qt_date = calendar->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    // Create the new snapshot
    finances::accounts::models::Snapshot new_snapshot_{
        .date_value = std::move(date),
        .amount = std::move(money_expected.value()),
    };

    utils::db::SnapshotManager manager{pool};
    auto r = manager.create(std::move(new_snapshot_));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account");
        // TODO: Communicate error to user
        return;
    }

    emit new_snapshot(account.id);

    this->accept();
}
