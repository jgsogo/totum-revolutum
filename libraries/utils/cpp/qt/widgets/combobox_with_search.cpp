#include "combobox_with_search.h"

#include <spdlog/spdlog.h>

#include <QAbstractItemView>
#include <QLineEdit>
#include <QTimer>

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

    // // Ensure focus is enabled for keyboard and clicks
    // this->setFocusPolicy(Qt::StrongFocus);
    // this->lineEdit()->setFocusPolicy(Qt::StrongFocus);

    // Whenever the text in the lineedit changes, update the filterd model
    connect(this->lineEdit(), &QLineEdit::textChanged, _sort_filter, &QSortFilterProxyModel::setFilterFixedString);

    // // Auto-show popup when typing (no focus loss)
    // _completer->popup()->setFocusPolicy(Qt::NoFocus); // popup won't steal keyboard focus
    // connect(this->lineEdit(), &QLineEdit::textEdited, [this]() { QTimer::singleShot(0, this, [this]() {
    //     // this->showPopup();
    //     if (!_completer->popup()->isVisible()) {
    //             // `complete()` uses the widget set earlier to resolve prefix
    //             _completer->complete();
    //         }
    //     this->lineEdit()->setFocus(Qt::OtherFocusReason); }); });

    // // TODO: Update current index in proxy when completer activates
    // connect(_completer, qOverload(&QCompleter::activated), this,
    //         [this](const QModelIndex& index) {
    //             SPDLOG_TRACE("Row {} selected", index.row());
    //             this->setCurrentIndex(index.row());
    //         });
    connect(this, &QComboBox::activated, [this](int index) {
        SPDLOG_TRACE("index {} actiovated", index);
        this->setCurrentIndex(index);
    });
}

void ComboBoxWithSearch::setModelColumn(int column) {
    _sort_filter->setFilterKeyColumn(column);
    _completer->setCompletionColumn(column);
    this->QComboBox::setModelColumn(column);
}

// void ComboBoxWithSearch::focusInEvent(QFocusEvent *event) {
//     SPDLOG_TRACE("ComboBoxWithSearch::focusInEvent");
//     this->QComboBox::focusInEvent(event);
//     this->showPopup();
// }

// void ComboBoxWithSearch::focusOutEvent(QFocusEvent *event) {
//     SPDLOG_TRACE("ComboBoxWithSearch::focusOutEvent");
//     this->QComboBox::focusOutEvent(event);
//     this->hidePopup();
// }

QSortFilterProxyModel* ComboBoxWithSearch::sortFilterProxyModel() { return _sort_filter; }

QCompleter* ComboBoxWithSearch::completer() { return _completer; }
