#pragma once

#include <QCalendarWidget>
#include <QComboBox>
#include <QDialog>
#include <QLineEdit>

#include <QDir>
#include <QStringListModel>

#include "libraries/utils/cpp/libpqxx/connection_pool.h"
#include "libraries/utils/cpp/qt/widgets/combobox_with_search.h"

#include "apps/finances/qt/tables/accounts.h"
#include "apps/finances/qt/tables/hierarchy_tree_columns.h"

#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/table_models/movement_type.h"

#include "apps/finances/qt/widgets/forms/amounts/movement_stacked_form.h"

class AddMovementWidget : public QDialog {
    Q_OBJECT

  public:
    explicit AddMovementWidget(utils::libpqxx::ConnectionPool& pool, AccountsTableModel<AccountColumns>& accounts,
                               MovementTypesTableModel<HierarchyTreeColumns>& movtypes, QWidget* parent = nullptr,
                               Qt::WindowFlags f = Qt::WindowFlags());
    virtual ~AddMovementWidget() = default;

  private slots:
    // The selected account has changed. The fields in the widget might change.
    void account_changed(const AccountModel&);

    // The selected movtype has changed. The fields in the widget might change.
    void movtype_changed(const MovementTypeModel&);

  public slots:
    void clear(bool keep_date = true);

    // Show the calendar to choose the date
    void show_mov_date();

    // Hide the calendar. Pass the date that will be used for this movement
    void hide_mov_date(QDate date);

    // When the calendar is hidden, any change to mov dates should be communicated from outside
    void mov_date_changed(QDate date);

  signals:
    // When the amount changes, someone outside might be interested on knowing it
    void amount_changed(finances::accounts::models::Amount);

  protected:
    utils::libpqxx::ConnectionPool& pool;

    utils::qt::widgets::ComboBoxWithSearch<AccountModel, AccountColumns>* account_combo;
    QComboBox* direction_combo;

    utils::qt::widgets::ComboBoxWithSearch<MovementTypeModel, HierarchyTreeColumns>* movtype_combo;
    QCalendarWidget* mov_date;

    widgets::forms::MovementStackedForm* mov_amount;
};
