#include "add_snapshot_numerable.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>
#include <spdlog/spdlog.h>

#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "apps/finances/qt/utils/utils.h"

using namespace finances::accounts::models;
using namespace finances::investments::models;

AddSnapshotNumerableWidget::AddSnapshotNumerableWidget(utils::libpqxx::ConnectionPool& pool_, const Account& account_,
                                                       QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_}, account{account_} {
    // components
    calendar = new QCalendarWidget(this);
    calendar->setVerticalHeaderFormat(QCalendarWidget::NoVerticalHeader);
    calendar->setFocusPolicy(Qt::StrongFocus);

    QRegularExpression rx(R"(^\d+(,\d{2})?$)");
    QRegularExpressionValidator* amount_validator = new QRegularExpressionValidator(rx, this);

    quantity = new QLineEdit(this);
    quantity->setValidator(amount_validator);
    quantity->setPlaceholderText("120,34");
    quantity->setFocusPolicy(Qt::StrongFocus);

    unit_value = new MoneyAmountEdit("Unit value (%1)", this);
    unit_value->setCcy(account.ccy);
    unit_value->setFocusPolicy(Qt::StrongFocus);

    // Layout
    QFormLayout* formLayout = new QFormLayout;
    formLayout->addRow(tr("&Date:"), calendar);
    formLayout->addRow(tr("&Quantity:"), quantity);
    formLayout->addRow(unit_value->get_label(), unit_value);

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Save | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddSnapshotNumerableWidget::add_snapshot_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QVBoxLayout* mainLayout = new QVBoxLayout;
    QString title(tr("%1 - Add snapshot numerable").arg(account.name));
    mainLayout->addWidget(new QLabel(title));
    mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
}

void AddSnapshotNumerableWidget::add_snapshot_clicked() {
    SPDLOG_DEBUG("AddSnapshotNumerableWidget::add_snapshot_clicked");

    auto unit_value_money_expected = unit_value->getMoneyAmount();
    if (!unit_value_money_expected) {
        // TODO: Communicate error to the user
        return;
    }

    if (!quantity->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    auto qt_date = calendar->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    auto quantity_amount = utils::qstring_to_amount(quantity->text(), account.ccy);
    if (!quantity_amount) {
        // TODO: Communicate error to the user
        return;
    }

    // Create the new snapshot
    Money amount = unit_value_money_expected.value() * quantity_amount.value();

    SnapshotNumerable new_snapshot_{
        .snapshot = Snapshot{.date_value = std::move(date), .amount = std::move(amount)},
        .quantity = std::move(quantity_amount.value()),
        .unit_value = std::move(unit_value_money_expected.value()),
    };

    utils::db::ModelData<SnapshotNumerable>::Manager manager{pool};
    auto r = manager.create(std::move(new_snapshot_));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account: {}", r.error());
        // TODO: Communicate error to user
        return;
    }

    emit new_snapshot(account.id);

    this->accept();
}
