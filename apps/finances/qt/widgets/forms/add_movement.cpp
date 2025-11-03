#include "add_movement.h"

#include <spdlog/spdlog.h>

#include <QAbstractItemView>
#include <QCompleter>
#include <QDialogButtonBox>
#include <QFormLayout>
#include <QGridLayout>

#include <QHeaderView>
#include <QSortFilterProxyModel>
#include <QStandardItemModel>
#include <QTableView>
#include <QTimer>
#include <QTreeView>

// namespace {
//     class NameFilterProxy : public QSortFilterProxyModel {
//         Q_OBJECT
//       public:
//         explicit NameFilterProxy(QObject* parent = nullptr) : QSortFilterProxyModel(parent) {}

//       protected:
//         // bool filterAcceptsRow(int source_row, const QModelIndex& source_parent) const override {
//         //     if (filterRegExp().isEmpty())
//         //         return true;
//         //     QModelIndex nameIndex =
//         //         sourceModel()->index(source_row, magic_enum::enum_integer(AccountColumns::NAME), source_parent);
//         //     QString name = nameIndex.data(Qt::DisplayRole).toString();
//         //     return name.contains(filterRegExp());
//         // }
//     };
// } // namespace

AddMovementWidget::AddMovementWidget(utils::libpqxx::ConnectionPool& pool,
                                     AccountsTableModel<AccountColumns>& accounts_, QWidget* parent, Qt::WindowFlags f)
    : QDialog(parent, f), pool{pool}, accounts{accounts_} {

    account_combo = new QComboBox;
    account_combo->setEditable(true);
    account_combo->setFocusPolicy(Qt::StrongFocus);
    {
        QCompleter* completer = new QCompleter(this);
        completer->setCaseSensitivity(Qt::CaseInsensitive);
        // completer->setMaxVisibleItems(4);
        QStandardItemModel* model = new QStandardItemModel(accounts.rowCount(), 3, completer);

        // Get a simplified model from the accounts, just the name and the custodian
        for (int i = 0; i < accounts.rowCount(); ++i) {
            const auto& item = accounts.get(i);

            QModelIndex nameIdx = model->index(i, 0);
            QModelIndex custodianIdx = model->index(i, 1);
            QModelIndex idIdx = model->index(i, 2);

            model->setData(nameIdx, QString::fromStdString(item.account.name));
            model->setData(custodianIdx, QString::fromStdString(item.account.custodian.second));
            model->setData(idIdx, QVariant::fromValue(item.id));
        }
        model->sort(0);

        completer->setModel(model);

        QTreeView* treeView = new QTreeView;
        completer->setPopup(treeView);
        treeView->hideColumn(2);
        treeView->setRootIsDecorated(false);
        treeView->header()->hide();
        treeView->header()->setStretchLastSection(false);
        treeView->header()->setSectionResizeMode(0, QHeaderView::Stretch);
        treeView->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);

        account_combo->setCompleter(completer);
        account_combo->setModel(model);
        // account_combo->setView(treeView);

        // Update current index in proxy when completer activates
        connect(completer, qOverload<const QModelIndex&>(&QCompleter::activated),
                [completer](const QModelIndex& index) {
                    SPDLOG_TRACE("(Completer) Row {} selected", index.row());
                    QVariant item_id = completer->completionModel()->data(index.siblingAtColumn(2));
                    SPDLOG_TRACE(" - id {}", item_id.toString().toStdString());
                });

        // Update current index in proxy when completer activates
        connect(account_combo, &QComboBox::activated, [this](int index) {
            SPDLOG_TRACE("(QCombobox) Row {} selected", index);
            QModelIndex idIdx = this->account_combo->model()->index(index, 2);
            QVariant item_id = this->account_combo->model()->data(idIdx);
            SPDLOG_TRACE(" - id {}", item_id.toString().toStdString());
        });
    }

    movtype = new QComboBox;

    mov_date = new QCalendarWidget;

    mov_amount = new widgets::forms::MovementStackedForm;

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QFormLayout* formLayout = new QFormLayout(this);
    formLayout->setFieldGrowthPolicy(QFormLayout::AllNonFixedFieldsGrow);
    formLayout->addRow(tr("&Account:"), account_combo);
    formLayout->addRow(tr("Movement &type:"), movtype);
    formLayout->addRow(tr("Movement &date:"), mov_date);
    formLayout->addRow(tr("&Amount:"), mov_amount);

    QGridLayout* layout = new QGridLayout;
    layout->addLayout(formLayout, 0, 0);
    layout->addWidget(buttonBox, 1, 0);
    this->setLayout(layout);

    account_combo->setFocus();
}

void AddMovementWidget::account_changed() { SPDLOG_DEBUG("AddMovementWidget::account_changed()"); }

void AddMovementWidget::movtype_changed() { SPDLOG_DEBUG("AddMovementWidget::movtype_changed()"); }

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }
