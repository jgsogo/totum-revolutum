#include "snapshot_non_numerable.h"

#include "utils.h"

tl::expected<SnapshotNonNumerable, std::string> SnapshotNonNumerable::from(QDate&& date, QString&& amount) {
    auto amount_amount = utils::qstring_to_amount(std::move(amount));
    if (!amount_amount) {
        return tl::unexpected{amount_amount.error()};
    }

    return {SnapshotNonNumerable{std::move(date), std::move(amount_amount.value())}};
}

SnapshotNonNumerable::SnapshotNonNumerable(QDate&& date, finances::accounts::models::Amount&& amount)
    : date_{std::move(date)}, amount_{std::move(amount)} {}

const QDate& SnapshotNonNumerable::date() const { return date_; }
const finances::accounts::models::Amount& SnapshotNonNumerable::amount() const { return amount_; };
