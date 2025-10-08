#include "add_snapshot.h"

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QLabel>
#include <QPushButton>
#include <QRegularExpression>
#include <QRegularExpressionValidator>
#include <QVBoxLayout>
#include <spdlog/spdlog.h>

#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

#include "apps/finances/qt/metatypes/utils.h"

AddSnapshotNumerableWidget::AddSnapshotNumerableWidget(utils::libpqxx::ConnectionPool& pool_,
                                                       const finances::accounts::models::Account& account_,
                                                       QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool_}, account{account_} {
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
    QString title(tr("%1 - Add snapshot numerable").arg(account.name));
    mainLayout->addWidget(new QLabel(title));
    mainLayout->addLayout(formLayout);
    mainLayout->addWidget(buttonBox);

    this->setLayout(mainLayout);
    this->setWindowTitle(title);
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

    auto qt_date = calendar->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    auto quantity_amount = utils::qstring_to_amount(quantity->text(), account.ccy);
    if (!quantity_amount) {
        // TODO: Communicate error to the user
        return;
    }

    auto unit_value_amount = utils::qstring_to_amount(unit_value->text(), account.ccy);
    if (!unit_value_amount) {
        // TODO: Communicate error to the user
        return;
    }

    finances::investments::models::SnapshotNumerableManager manager{pool};
    auto r = manager.create(account.id, std::move(date), std::move(quantity_amount.value()),
                            std::move(unit_value_amount.value()));
    if (!r) {
        SPDLOG_ERROR("Error adding snapshot to account");
        // TODO: Communicate error to user
        return;
    }

    emit new_snapshot(account.id);

    this->accept();
}
