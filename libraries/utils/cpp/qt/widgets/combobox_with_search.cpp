#include "combobox_with_search.h"

#include <QLineEdit>

using namespace utils::qt::widgets;

ComboBoxWithSearch::ComboBoxWithSearch(QAbstractItemModel* model, QWidget* parent) : QComboBox(parent) {

    _sort_filter = new QSortFilterProxyModel;
    _sort_filter->setSourceModel(model);
    _sort_filter->setSortCaseSensitivity(Qt::CaseInsensitive);
    _sort_filter->setFilterCaseSensitivity(Qt::CaseInsensitive);

    _completer = new QCompleter(_sort_filter, this);
    _completer->setCaseSensitivity(Qt::CaseInsensitive);
    _completer->setFilterMode(Qt::MatchContains);
    _completer->setCompletionMode(QCompleter::PopupCompletion);

    this->setEditable(true);
    this->setModel(_sort_filter);
    this->setCompleter(_completer);

    // Ensure focus is enabled for keyboard and clicks
    this->setFocusPolicy(Qt::StrongFocus);
    this->lineEdit()->setFocusPolicy(Qt::StrongFocus);

    // Live filter as the user types
    connect(this->lineEdit(), &QLineEdit::textChanged, _sort_filter, &QSortFilterProxyModel::setFilterFixedString);
}

void ComboBoxWithSearch::setModelColumn(int column) {
    _sort_filter->setFilterKeyColumn(column);
    _completer->setCompletionColumn(column);
    this->QComboBox::setModelColumn(column);
}

QSortFilterProxyModel* ComboBoxWithSearch::sortFilterProxyModel() { return _sort_filter; }

QCompleter* ComboBoxWithSearch::completer() { return _completer; }
