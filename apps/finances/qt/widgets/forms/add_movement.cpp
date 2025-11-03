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

    account_combo = new QLineEdit;
    {
        QCompleter* completer = new QCompleter(this);
        completer->setMaxVisibleItems(4);
        {
            // Get a simplified model from the accounts, just the name and the custodian
            QStandardItemModel* model = new QStandardItemModel(accounts.rowCount(), 2, completer);
            for (int i = 0; i < accounts.rowCount(); ++i) {
                const auto& item = accounts.get(i);

                QModelIndex nameIdx = model->index(i, 0);
                QModelIndex custodianIdx = model->index(i, 1);

                model->setData(nameIdx, QString::fromStdString(item.account.name));
                model->setData(custodianIdx, QString::fromStdString(item.account.custodian.second));
            }
            completer->setModel(model);

            QTreeView* treeView = new QTreeView;
            completer->setPopup(treeView);
            treeView->setRootIsDecorated(false);
            treeView->header()->hide();
            treeView->header()->setStretchLastSection(false);
            treeView->header()->setSectionResizeMode(0, QHeaderView::Stretch);
            treeView->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
        }

        account_combo->setCompleter(completer);

        // auto* proxy = new QSortFilterProxyModel;
        // proxy->setSourceModel(&accounts);
        // proxy->setSortCaseSensitivity(Qt::CaseInsensitive);
        // proxy->sort(magic_enum::enum_integer(AccountColumns::NAME));
        // proxy->setFilterCaseSensitivity(Qt::CaseInsensitive);
        // // proxy->setFilterKeyColumn(magic_enum::enum_integer(AccountColumns::NAME));
        // proxy->setFilterKeyColumn(-1);

        // QCompleter* completer = new QCompleter(proxy, account_combo);
        // completer->setCompletionColumn(magic_enum::enum_integer(AccountColumns::NAME));
        // // completer->setCompletionMode(QCompleter::PopupCompletion);  // Auto-shows popup on typing
        // completer->setCompletionMode(QCompleter::UnfilteredPopupCompletion);
        // completer->setCaseSensitivity(Qt::CaseInsensitive);
        // completer->setFilterMode(Qt::MatchContains); // Optional: Match anywhere
        // completer->setModelSorting(QCompleter::CaseInsensitivelySortedModel);
        // account_combo->setCompleter(completer);

        // // Live filter as the user types
        // connect(account_combo, &QLineEdit::textEdited, proxy, &QSortFilterProxyModel::setFilterFixedString);

        // // Update current index in proxy when completer activates
        // connect(completer, qOverload<const QModelIndex&>(&QCompleter::activated),
        //         [this, completer](const QModelIndex& index) {
        //             SPDLOG_TRACE("Row {} selected", index.row());
        //             auto complete_model = completer->completionModel();
        //             QVariant account_name =
        //                 complete_model->data(index.siblingAtColumn(magic_enum::enum_integer(AccountColumns::NAME)));

        //             // auto path = completer->pathFromIndex(index);
        //             SPDLOG_TRACE(" - account_name: {}", account_name.toString().toStdString());
        //             this->account_combo->clear();
        //             this->account_combo->setText(account_name.toString());
        //             // account_combo->setText("lolololo");

        //             // auto model_index = proxy->mapToSource(index);

        //             // QVariant account_name =
        //             //     proxy->data(model_index.siblingAtColumn(magic_enum::enum_integer(AccountColumns::NAME)));
        //         });
    }
    // account_combo->setModelColumn(magic_enum::enum_integer(AccountColumns::NAME));
    {
        // auto* proxy = new QSortFilterProxyModel;
        // proxy->setSourceModel(&accounts);
        // proxy->setSortCaseSensitivity(Qt::CaseInsensitive);
        // proxy->setFilterCaseSensitivity(Qt::CaseInsensitive);
        // proxy->setFilterKeyColumn(magic_enum::enum_integer(AccountColumns::NAME));

        // account_combo->setEditable(true);
        // account_combo->setModel(proxy);
        // account_combo->setModelColumn(magic_enum::enum_integer(AccountColumns::NAME));

        // QCompleter* completer = new QCompleter(proxy, account_combo);
        // completer->setCompletionColumn(magic_enum::enum_integer(AccountColumns::NAME));
        // completer->setCaseSensitivity(Qt::CaseInsensitive);
        // completer->setFilterMode(Qt::MatchContains);
        // completer->setCompletionMode(QCompleter::PopupCompletion);

        // // Keep focus on line edit while popup is shown
        // completer->popup()->setFocusPolicy(Qt::NoFocus);
        // completer->setWidget(account_combo->lineEdit());
        // account_combo->setCompleter(completer);

        // // Ensure focus is enabled
        // account_combo->setFocusPolicy(Qt::StrongFocus);
        // account_combo->lineEdit()->setFocusPolicy(Qt::StrongFocus);

        // // Live filter as the user types
        // connect(account_combo->lineEdit(), &QLineEdit::textEdited, proxy,
        // &QSortFilterProxyModel::setFilterFixedString);

        // // Auto-show popup when typing (no focus loss)
        // connect(account_combo->lineEdit(), &QLineEdit::textEdited, account_combo,
        //         [combo = account_combo]() { QTimer::singleShot(0, combo, [combo]() { combo->showPopup(); }); });

        // // Update current index in proxy when completer activates
        // connect(completer, QOverload<const QModelIndex&>::of(&QCompleter::activated),
        //         [combo = account_combo](const QModelIndex& index) {
        //             SPDLOG_TRACE("Row {} selected", index.row());
        //             combo->setCurrentIndex(index.row());
        //         });

        // // Nice layout
        // // 🔧 Fixed size line edit
        // auto* line_edit = account_combo->lineEdit();
        // line_edit->setFixedWidth(300); // adjust as you like
        // line_edit->setMinimumWidth(300);
        // line_edit->setAlignment(Qt::AlignLeft);
        // // line_edit->setFocusPolicy(Qt::StrongFocus);

        // account_combo->view()->setMinimumWidth(300);
        // account_combo->setSizeAdjustPolicy(QComboBox::AdjustToContents);
        // account_combo->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Preferred);
        // // account_combo->lineEdit()->setFocusPolicy(Qt::StrongFocus);
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
