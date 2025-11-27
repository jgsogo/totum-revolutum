#include <QFontDatabase>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QScreen>
#include <QSortFilterProxyModel>
#include <QTableView>
#include <QThread>
#include <QVBoxLayout>
#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QStylePainter>
#include <spdlog/spdlog.h>

#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/utils/cpp/libpqxx/connection_pool.h"

#include "apps/finances/qt/db/notificator.h"

#include "apps/finances/qt/metatypes/types.h"
#include "apps/finances/qt/table_models/accounts.h"
#include "apps/finances/qt/table_models/movement_type.h"
#include "apps/finances/qt/tables/accounts.h"
#include "apps/finances/qt/tables/hierarchy_tree_columns.h"
#include "apps/finances/qt/version.hpp"
#include "apps/finances/qt/widgets/accounts/main_tab.h"
#include "apps/finances/qt/widgets/forms/add_transaction.h"

int main(int argc, char** argv) {
    spdlog::set_level(spdlog::level::trace); // TODO: Configurable via CLI and/or envvar
    spdlog::set_pattern("[%Y-%m-%d %H:%M:%S.%e][%^%8l%$] %v (%@)");

    auto pool_expected = utils::libpqxx::ConnectionPool::from_env("FINANCES_QT_", 4);
    if (!pool_expected) {
        SPDLOG_ERROR("Failed to connect to the database: {}", pool_expected.error());
        return 1;
    }
    utils::libpqxx::ConnectionPool& pool = pool_expected.value();

    QApplication app(argc, argv);
    QWidget window;

    register_metatypes();

    // FIXME: Make the pool only available to the models. Every DB operation should
    //        be performed through these classes. This will require some metatypes
    //        to send the info throught the QT channels.

    // Get who I am
    std::optional<finances::accounts::models::AccountHolder> me = std::nullopt;
    {
        const char* initial_holder_pk = std::getenv("FINANCES_QT_INITIAL_HOLDER_PK");
        if (initial_holder_pk != nullptr) {
            utils::db::Id me_id{utils::db::IdType{std::stoull(initial_holder_pk)}};
            utils::db::ModelManager<finances::accounts::models::AccountHolder> manager{pool};
            auto me_expected = manager.get(me_id);
            if (!me_expected) {
                SPDLOG_ERROR("Cannot retrieve AccountHolder for pk={}", me_id);
                return 1;
            }
            me = std::move(me_expected.value());
            SPDLOG_INFO("AccountHolder for ME: pk={}, name={}", me->id, me->name);
        }
    }

    // Create the long-living models
    AccountsTableModel<AccountColumns> accounts_tablemodel{pool};
    MovementTypesTableModel<HierarchyTreeColumns> movement_types_tablemodel{pool};

    // Run a notificator that will monitor notifications from the database
    auto conn = pool.acquire();
    std::chrono::milliseconds ms{1000};
    Notificator notificator{std::move(*conn), ms};
    QObject::connect(&notificator, &Notificator::account_changed, &accounts_tablemodel,
                     &utils::qt::models::_detail::GenericTableModel::refresh_item);

    // Create the tabs for the accounts
    MainTabWidget* tabWidget = new MainTabWidget(pool, me, accounts_tablemodel, movement_types_tablemodel);
    QObject::connect(tabWidget, &MainTabWidget::account_changed, &notificator, &Notificator::notify_account);
    QObject::connect(&notificator, &Notificator::account_changed, tabWidget, &MainTabWidget::on_account_changed);

    // - popup - add transaction
    QPushButton* bt_add_transaction = new QPushButton(QObject::tr("Add transaction"));
    {
        // QObject::connect(bt_add_transaction, &QPushButton::clicked, popup_add_transaction, &QDialog::open);
        QObject::connect(bt_add_transaction, &QPushButton::clicked,
                         [&pool, &accounts_tablemodel, &movement_types_tablemodel, &notificator]() {
                             AddTransactionWidget* popup_add_transaction =
                                 new AddTransactionWidget(pool, accounts_tablemodel, movement_types_tablemodel);
                             popup_add_transaction->setModal(true);
                             popup_add_transaction->setSizeGripEnabled(true);
                             QObject::connect(popup_add_transaction, &AddTransactionWidget::new_movement, &notificator,
                                              &Notificator::notify_account);
                             popup_add_transaction->open();
                         });
    }

    QVBoxLayout* layout = new QVBoxLayout();
    layout->addWidget(bt_add_transaction);
    layout->addWidget(tabWidget);

    window.setLayout(layout);
    window.setWindowTitle(QString::fromStdString(std::format("Finances v{}", FINANCES_VERSION)));
    QSize screen_size = QGuiApplication::primaryScreen()->availableGeometry().size();
    window.resize(screen_size.width() * 0.5, screen_size.height());
    window.show();

    return app.exec();
}
