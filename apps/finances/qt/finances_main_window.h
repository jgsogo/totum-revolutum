#pragma once

#include <QMainWindow>

QT_BEGIN_NAMESPACE
namespace Ui {
    class FinancesMainWindow;
}
QT_END_NAMESPACE

class FinancesMainWindow : public QMainWindow {
    Q_OBJECT

  public:
    explicit FinancesMainWindow(QWidget* parent = nullptr);
    ~FinancesMainWindow();

  private:
    Ui::FinancesMainWindow* ui;
    QString currentFile;
};
