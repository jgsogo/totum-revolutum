#include "account_detail.h"

#include <QCheckBox>
#include <QGroupBox>
#include <QLabel>
#include <QLineEdit>
#include <QTableView>
#include <QTimer>
#include <QVBoxLayout>

AccountDetailWidget::AccountDetailWidget(utils::libpqxx::ConnectionPool& pool, QWidget* parent, Qt::WindowFlags f)
    : QWidget(parent, f) {}
