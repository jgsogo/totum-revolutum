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
#include "apps/finances/qt/models/accounts/table.h"
#include "apps/finances/qt/models/accounts_table.h"
#include "apps/finances/qt/models/movement_type.h"
#include "apps/finances/qt/version.hpp"
#include "apps/finances/qt/widgets/accounts/main_tab.h"

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

    // Create the main model with the accounts
    AccountTableModel* model = new AccountTableModel(pool);
    MovementTypeTableModel* movtype_model = new MovementTypeTableModel(pool);

    // Run a notificator that will monitor notifications from the database
    auto conn = pool.acquire();
    std::chrono::milliseconds ms{1000};
    Notificator notificator{std::move(*conn), ms};
    QObject::connect(&notificator, &Notificator::account_changed, model, &AccountTableModel::fetch_snapshot);

    // Create the tabs for the accounts
    MainTabWidget* tabWidget = new MainTabWidget(pool, me, model, movtype_model);
    QObject::connect(tabWidget, &MainTabWidget::account_changed, &notificator, &Notificator::notify_account);

    QVBoxLayout* layout = new QVBoxLayout();
    layout->addWidget(tabWidget);

    // AccountsTable* accounts = new AccountsTable{pool};
    // QTableView* table_view = new QTableView;
    // table_view->setModel(accounts);
    // layout->addWidget(table_view);

    window.setLayout(layout);
    window.setWindowTitle(QString::fromStdString(std::format("Finances v{}", FINANCES_VERSION)));
    QSize screen_size = QGuiApplication::primaryScreen()->availableGeometry().size();
    window.resize(screen_size.width() * 0.5, screen_size.height());
    window.show();

    return app.exec();
}
