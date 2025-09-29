#include "snapshot_numerable.h"

#include "utils.h"

tl::expected<SnapshotNumerable, std::string> SnapshotNumerable::from(QDate&& date, QString&& quantity,
                                                                     QString&& unit_value) {
    auto quantity_amount = utils::qstring_to_amount(std::move(quantity));
    if (!quantity_amount) {
        return tl::unexpected{quantity_amount.error()};
    }

    auto unit_value_amount = utils::qstring_to_amount(std::move(unit_value));
    if (!unit_value_amount) {
        return tl::unexpected{unit_value_amount.error()};
    }

    return {
        SnapshotNumerable{std::move(date), std::move(quantity_amount.value()), std::move(unit_value_amount.value())}};
}

SnapshotNumerable::SnapshotNumerable(QDate&& date, finances::accounts::models::Amount&& quantity,
                                     finances::accounts::models::Amount&& unit_value)
    : date_{std::move(date)}, quantity_{std::move(quantity)}, unit_value_{std::move(unit_value)} {}

const QDate& SnapshotNumerable::date() const { return date_; }
const finances::accounts::models::Amount& SnapshotNumerable::quantity() const { return quantity_; };
const finances::accounts::models::Amount& SnapshotNumerable::unit_value() const { return unit_value_; };
