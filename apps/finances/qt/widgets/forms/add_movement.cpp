#include "add_movement.h"

#include <spdlog/spdlog.h>

#include <QDialogButtonBox>
#include <QFormLayout>
#include <QGridLayout>

#include "libraries/finances/accounts/cpp/models/movement.h"

AddMovementWidget::AddMovementWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>& accounts,
                                     MovementTypesTableModel<HierarchyTreeColumns>& movtypes, QWidget* parent,
                                     Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool} {

    account_combo =
        new utils::qt::widgets::ComboBoxWithSearch{accounts, AccountColumns::ID, AccountColumns::NAME, parent};

    direction_combo = new QComboBox;
    direction_combo->setFocusPolicy(Qt::StrongFocus);
    for (auto dirname : magic_enum::enum_names<finances::accounts::models::MovementDirection>()) {
        direction_combo->addItem(QString::fromStdString(std::string(dirname)));
    }

    movtype_combo = new utils::qt::widgets::ComboBoxWithSearch{movtypes, HierarchyTreeColumns::ID,
                                                               HierarchyTreeColumns::BREADCRUMB, parent};

    mov_date = new QCalendarWidget;

    mov_amount = new widgets::forms::MovementStackedForm;

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &AddMovementWidget::add_movement_clicked);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QFormLayout* formLayout = new QFormLayout;
    formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
    formLayout->addRow(tr("&Account:"), account_combo);
    formLayout->addRow(tr("&Direction:"), direction_combo);
    formLayout->addRow(tr("Movement &type:"), movtype_combo);
    formLayout->addRow(tr("Movement &date:"), mov_date);

    QGridLayout* layout = new QGridLayout;
    layout->addLayout(formLayout, 0, 0);
    layout->addWidget(mov_amount, 1, 0);
    layout->addWidget(buttonBox, 2, 0);
    this->setLayout(layout);

    connect(account_combo, &utils::qt::widgets::_ComboBoxWithSearch::activated, [this, &accounts](utils::db::Id id) {
        auto account_expected = accounts.get(id);
        if (!account_expected) {
            SPDLOG_ERROR("Cannot get an account with id '{}'", id);
            // TODO: Communicate error to the user
            return;
        }
        this->account_changed(account_expected.value());
    });

    connect(movtype_combo, &utils::qt::widgets::_ComboBoxWithSearch::activated, [this, &movtypes](utils::db::Id id) {
        auto movtypes_expected = movtypes.get(id);
        if (!movtypes_expected) {
            SPDLOG_ERROR("Cannot get a movtype with id '{}'", id);
            // TODO: Communicate error to the user
            return;
        }
        this->movtype_changed(movtypes_expected.value());
    });

    account_combo->setFocus();
}

void AddMovementWidget::clear(bool keep_date) {
    this->account_combo->clearEditText();
    this->movtype_combo->clearEditText();
    this->mov_amount->clear();
    if (!keep_date) {
        this->mov_date->setSelectedDate(QDate::currentDate());
    }
}

void AddMovementWidget::account_changed(const AccountModel& account) {
    SPDLOG_DEBUG("AddMovementWidget::account_changed(account.name={})", account.account.name);
    mov_amount->set_ccy(account.account.ccy);
    if (account.account.is_numerable) {
        mov_amount->set_movement_numerable();
    } else {
        mov_amount->set_movement_non_numerable();
    }
}

void AddMovementWidget::movtype_changed(const MovementTypeModel&) {
    SPDLOG_DEBUG("AddMovementWidget::movtype_changed()");
}

void AddMovementWidget::add_movement_clicked() {
    SPDLOG_DEBUG("AddMovementWidget::add_movement_clicked()");

    // - movement type
    auto movtype_expected = movtype_combo->selected();
    if (!movtype_expected) {
        // TODO: Tell the user about the error
        return;
    }
    if (!movtype_expected.value()) {
        // TODO: Tell the user about the error
        return;
    }
    MovementTypeModel movtype = std::move(movtype_expected.value().value());

    // - account
    auto account_expected = account_combo->selected();
    if (!account_expected) {
        // TODO: Tell the user about the error
        return;
    }
    if (!account_expected.value()) {
        // TODO: Tell the user about the error
        return;
    }
    AccountModel account = std::move(account_expected.value().value());

    // - date
    auto qt_date = mov_date->selectedDate();
    utils::libpqxx::Date date{date::year_month_day{date::year{qt_date.year()},
                                                   date::month{static_cast<unsigned int>(qt_date.month())},
                                                   date::day{static_cast<unsigned int>(qt_date.day())}}};

    // - direction
    // FIXME: Take this value!
    auto direction_expected =
        magic_enum::enum_cast<finances::accounts::models::MovementDirection>(direction_combo->currentIndex());
    if (!direction_expected) {
        // TODO: Tell the user about the error
        return;
    }

    // - amount
    auto amount_expected = mov_amount->getMoneyAmount();
    if (!amount_expected) {
        // TODO: Tell the user about the error
        return;
    }

    // Create the base movement with the data in this form
    finances::accounts::models::Movement movement{
        .id = {std::monostate{}}, // No id, it's not in the database yet!
        .transaction = std::make_pair(utils::db::Id{std::monostate{}}, std::string{""}),
        .type = std::make_pair(movtype.id, movtype.movtype.name),
        .direction = direction_expected.value(),
        .account = std::make_pair(account.id, account.account.name),
        .date_value = date,
        // .fx =
        .amount = std::move(amount_expected.value()),
    };

    // Now, we need to get additional data from the amounts, in case they were other type of movements
    auto final_movement_expected = mov_amount->populateAdditionalData(std::move(movement));
    if (!final_movement_expected) {
        // TODO: Tell the user about the error
        return;
    }

    MovementModel movement_model{.id = movement.id,
                                 .movement = std::move(final_movement_expected.value()),
                                 .movtype_breadcrumb = movtype.breadcrumb};

    emit new_movement(movement_model);
    this->accept();
}

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }
