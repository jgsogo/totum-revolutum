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

    QRegularExpression rx(R"(^\d+(,\d{4})?$)");
    QRegularExpressionValidator* quantity_validator = new QRegularExpressionValidator(rx, this);

    quantity = new QLineEdit(this);
    quantity->setValidator(quantity_validator);
    quantity->setPlaceholderText("120,34");
    connect(quantity, &QLineEdit::textEdited, this, &AddSnapshotNumerableWidget::on_inputs_changed);

    unit_value = new MoneyAmountEdit("Unit value (%1)", this);
    unit_value->setCcy(account.ccy);
    connect(unit_value, &QLineEdit::textEdited, this, &AddSnapshotNumerableWidget::on_inputs_changed);

    amount_label = new QLabel(tr("Amount (%1):").arg(static_cast<std::string>(account.ccy)));

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
    mainLayout->addWidget(amount_label);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
}

void AddSnapshotNumerableWidget::on_inputs_changed() {
    auto amount_expected = this->getMoneyAmount();
    if (amount_expected) {
        this->amount_label->setText(
            QString("Amount (%1): %2")
                .arg(static_cast<std::string>(account.ccy), static_cast<std::string>(amount_expected.value())));
    }
}

ExpectedType<Money> AddSnapshotNumerableWidget::getMoneyAmount() const {
    SPDLOG_DEBUG("AddSnapshotNumerableWidget::getMoneyAmount");

    auto unit_value_money_expected = unit_value->getMoneyAmount();
    if (!unit_value_money_expected) {
        return tl::unexpected{unit_value_money_expected.error()};
    }

    if (!quantity->hasAcceptableInput()) {
        return tl::unexpected{::error::InputFieldNotSet{"quantity"}};
    }

    auto quantity_amount = utils::qstring_to_amount(quantity->text(), account.ccy);
    if (!quantity_amount) {
        return tl::unexpected{quantity_amount.error()};
    }

    // Create the new snapshot
    Money amount = unit_value_money_expected.value() * quantity_amount.value();
    return {std::move(amount)};
}

void AddSnapshotNumerableWidget::add_snapshot_clicked() {
    SPDLOG_DEBUG("AddSnapshotNumerableWidget::add_snapshot_clicked");

    // Create the new snapshot
    auto amount_expected = this->getMoneyAmount();
    if (!amount_expected) {
        // TODO: Communicate error to the user
        return;
    }

    auto unit_value_money_expected = unit_value->getMoneyAmount();
    if (!unit_value_money_expected) {
        // TODO: Communicate error to the user
        return;
    }

    if (!quantity->hasAcceptableInput()) {
        // TODO: Communicate error to the user
        return;
    }

    auto quantity_amount = utils::qstring_to_amount(quantity->text(), account.ccy);
    if (!quantity_amount) {
        // TODO: Communicate error to the user
        return;
    }

    auto qt_date = calendar->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    SnapshotNumerable new_snapshot_{
        .snapshot = Snapshot{.account = std::make_pair(account.id, account.name),
                             .date_value = std::move(date),
                             .amount = std::move(amount_expected.value())},
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
