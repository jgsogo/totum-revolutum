#include "add_movement.h"

#include <spdlog/spdlog.h>

#include <QAbstractItemView>
#include <QCompleter>
#include <QDialogButtonBox>
#include <QFormLayout>
#include <QVBoxLayout>

#include <QHeaderView>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QTimer>

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
    {
        auto* proxy = new QSortFilterProxyModel;
        proxy->setSourceModel(&accounts);
        proxy->setSortCaseSensitivity(Qt::CaseInsensitive);
        proxy->setFilterCaseSensitivity(Qt::CaseInsensitive);
        proxy->setFilterKeyColumn(magic_enum::enum_integer(AccountColumns::NAME));

        account_combo->setEditable(true);
        account_combo->setModel(proxy);
        account_combo->setModelColumn(magic_enum::enum_integer(AccountColumns::NAME));

        QCompleter* completer = new QCompleter(proxy, account_combo);
        completer->setCompletionColumn(magic_enum::enum_integer(AccountColumns::NAME));
        completer->setCaseSensitivity(Qt::CaseInsensitive);
        completer->setFilterMode(Qt::MatchContains);
        completer->setCompletionMode(QCompleter::PopupCompletion);

        // Keep focus on line edit while popup is shown
        completer->popup()->setFocusPolicy(Qt::NoFocus);
        completer->setWidget(account_combo->lineEdit());
        account_combo->setCompleter(completer);

        // Ensure focus is enabled
        account_combo->setFocusPolicy(Qt::StrongFocus);
        account_combo->lineEdit()->setFocusPolicy(Qt::StrongFocus);

        // Live filter as the user types
        connect(account_combo->lineEdit(), &QLineEdit::textEdited, proxy, &QSortFilterProxyModel::setFilterFixedString);

        // Auto-show popup when typing (no focus loss)
        connect(account_combo->lineEdit(), &QLineEdit::textEdited, account_combo,
                [combo = account_combo]() { QTimer::singleShot(0, combo, [combo]() { combo->showPopup(); }); });

        // Update current index in proxy when completer activates
        connect(completer, QOverload<const QModelIndex&>::of(&QCompleter::activated),
                [combo = account_combo](const QModelIndex& index) {
                    SPDLOG_TRACE("Row {} selected", index.row());
                    combo->setCurrentIndex(index.row());
                });

        // Nice layout
        // 🔧 Fixed size line edit
        auto* line_edit = account_combo->lineEdit();
        line_edit->setFixedWidth(300); // adjust as you like
        line_edit->setMinimumWidth(300);
        line_edit->setAlignment(Qt::AlignLeft);
        // line_edit->setFocusPolicy(Qt::StrongFocus);

        account_combo->view()->setMinimumWidth(300);
        account_combo->setSizeAdjustPolicy(QComboBox::AdjustToContents);
        account_combo->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Preferred);
        // account_combo->lineEdit()->setFocusPolicy(Qt::StrongFocus);
    }

    movtype = new QComboBox;

    mov_date = new QCalendarWidget;

    mov_amount = new widgets::forms::MovementStackedForm;

    QDialogButtonBox* buttonBox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
    connect(buttonBox, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttonBox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    QFormLayout* formLayout = new QFormLayout;
    formLayout->addRow(tr("&Account"), account_combo);
    formLayout->addRow(tr("Movement type"), movtype);
    formLayout->addRow(tr("Movement date"), mov_date);
    formLayout->addRow(tr("Amount"), mov_amount);

    QVBoxLayout* layout = new QVBoxLayout;
    layout->addLayout(formLayout);
    layout->addWidget(buttonBox);
    this->setLayout(layout);
}

void AddMovementWidget::account_changed() { SPDLOG_DEBUG("AddMovementWidget::account_changed()"); }

void AddMovementWidget::movtype_changed() { SPDLOG_DEBUG("AddMovementWidget::movtype_changed()"); }

void AddMovementWidget::show_mov_date() { SPDLOG_DEBUG("AddMovementWidget::show_mov_date()"); }

void AddMovementWidget::hide_mov_date(QDate date) { SPDLOG_DEBUG("AddMovementWidget::hide_mov_date()"); }

void AddMovementWidget::mov_date_changed(QDate date) { SPDLOG_DEBUG("AddMovementWidget::mov_date_changed()"); }
