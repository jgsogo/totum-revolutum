#include "money_amount.h"

#include <spdlog/spdlog.h>

#include <QLabel>
#include <QRegularExpression>
#include <QRegularExpressionValidator>

#include "apps/finances/qt/utils/utils.h"

struct MoneyAmountEdit::Impl {
    QLabel* label;
    const char* label_template;
    std::optional<finances::accounts::models::Ccy> ccy;
};

MoneyAmountEdit::MoneyAmountEdit(const char* label_template_, QWidget* parent)
    : QLineEdit{parent}, pImpl{std::make_unique<MoneyAmountEdit::Impl>()} {
    pImpl->label_template = label_template_;
    pImpl->label = new QLabel(pImpl->label_template);

    this->setEnabled(false);

    connect(this, &QLineEdit::textEdited, this, &MoneyAmountEdit::on_text_edited);
}

MoneyAmountEdit::~MoneyAmountEdit() = default;

QLabel* MoneyAmountEdit::get_label() const { return pImpl->label; }

void MoneyAmountEdit::setCcy(finances::accounts::models::Ccy ccy) {
    SPDLOG_DEBUG("MoneyAmountEdit::setCcy(ccy={})", ccy);
    pImpl->ccy = ccy;

    // Depending on the CCY, we might have different formats here
    QRegularExpression rx(R"(^\d+(,\d{4})?$)");
    QRegularExpressionValidator* amount_validator = new QRegularExpressionValidator(rx, this);
    this->setValidator(amount_validator);
    this->setPlaceholderText("120,34");
    this->setEnabled(true);

    pImpl->label->setText(tr(pImpl->label_template).arg(static_cast<std::string>(ccy)));
}

void MoneyAmountEdit::noCcy() {
    SPDLOG_DEBUG("MoneyAmountEdit::noCcy()");
    pImpl->ccy = std::nullopt;
    this->setEnabled(false);
    pImpl->label->setText(tr(pImpl->label_template).arg("<unknown>"));
}

ExpectedType<finances::accounts::models::Money> MoneyAmountEdit::getMoneyAmount() const {
    SPDLOG_DEBUG("MoneyAmountEdit::getMoneyAmount()");

    if (!pImpl->ccy.has_value()) {
        return tl::unexpected{error::InputFieldNotSet{"ccy"}};
    }

    auto amount_expected = utils::qstring_to_amount(this->text(), pImpl->ccy.value());
    if (!amount_expected) {
        return tl::unexpected{amount_expected.error()};
    }

    finances::accounts::models::Money money{std::move(amount_expected.value()), pImpl->ccy.value()};
    return {std::move(money)};
}

void MoneyAmountEdit::on_text_edited(const QString& text) {
    SPDLOG_DEBUG("MoneyAmountEdit::on_text_edited(text={})", text.toStdString());
    assert(pImpl->ccy.has_value() && "MoneyAmountEdit cannot be changed unless it has a Ccy set");

    auto money_amount_expected = this->getMoneyAmount();
    if (!money_amount_expected) {
        SPDLOG_WARN("We are skipping this signal: {}", money_amount_expected.error());
        return;
    }

    emit money_changed(std::move(money_amount_expected.value()));
}
