#pragma once

#include "tl/expected.hpp"
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/amount.h"

class SnapshotNumerable {
    // using Amount = finances::accounts::models::Amount;

  public:
    SnapshotNumerable() = default;
    ~SnapshotNumerable() = default;
    SnapshotNumerable(const SnapshotNumerable&) = default;
    SnapshotNumerable& operator=(const SnapshotNumerable&) = default;

    static tl::expected<SnapshotNumerable, std::string> from(QDate&& date, QString&& quantity, QString&& unit_value);

    SnapshotNumerable(QDate&& date, finances::accounts::models::Amount&& quantity,
                      finances::accounts::models::Amount&& unit_value);

    const QDate& date() const;
    const finances::accounts::models::Amount& quantity() const;
    const finances::accounts::models::Amount& unit_value() const;

  private:
    QDate date_;
    finances::accounts::models::Amount quantity_;
    finances::accounts::models::Amount unit_value_;
};

Q_DECLARE_METATYPE(SnapshotNumerable);
