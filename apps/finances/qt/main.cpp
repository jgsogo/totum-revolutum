#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QScreen>
#include <QTableView>
#include <QThread>
#include <QVBoxLayout>
#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QStylePainter>
#include <spdlog/spdlog.h>

#include "libraries/utils/cpp/db/connection_pool.h"

#include "apps/finances/qt/models/accounts_table.h"
#include "apps/finances/qt/version.hpp"
#include "apps/finances/qt/widgets/sidebar.h"

int main(int argc, char** argv) {
    spdlog::set_level(spdlog::level::debug); // TODO: Configurable via CLI and/or envvar
    spdlog::set_pattern("[%Y-%m-%d %H:%M:%S.%e][%^%8l%$][engine] %v (%@)");

    auto pool = utils::db::ConnectionPool::from_env("FINANCES_QT_", 4);

    QApplication app(argc, argv);
    QWidget window;

    QHBoxLayout* all = new QHBoxLayout();
    // QVBoxLayout* left_pane = new QVBoxLayout();
    // QVBoxLayout* main_pane = new QVBoxLayout();

    // all->addLayout(left_pane, 20);
    // all->addLayout(main_pane);

    // left_pane->addWidget(createSidebar());
    // QIcon undoicon = QIcon::fromTheme(QIcon::ThemeIcon::EditUndo);
    all->addWidget(new SideBar(), 30);

    QTableView* table = new QTableView();
    AccountTableModel* table_model = AccountTableModel::create_with_all(pool);
    table->setModel(table_model);
    all->addWidget(table, 70);

    window.setLayout(all);
    // Set up the model and configure the view...
    // QApplication::translate("finances", "Finances")
    window.setWindowTitle(QString::fromStdString(std::format("Finances v{}", FINANCES_VERSION)));
    QSize screen_size = QGuiApplication::primaryScreen()->availableGeometry().size();
    window.resize(screen_size.width() * 0.5, screen_size.height());
    window.show();

    return app.exec();
}
