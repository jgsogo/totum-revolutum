#pragma once

#include "tl/expected.hpp"
#include <QDate>

#include "libraries/finances/accounts/cpp/models/types/amount.h"

class SnapshotNonNumerable {
  public:
    SnapshotNonNumerable() = default;
    ~SnapshotNonNumerable() = default;
    SnapshotNonNumerable(const SnapshotNonNumerable&) = default;
    SnapshotNonNumerable& operator=(const SnapshotNonNumerable&) = default;

    static tl::expected<SnapshotNonNumerable, std::string> from(QDate&& date, QString&& amount);

    SnapshotNonNumerable(QDate&& date, finances::accounts::models::Amount&& amount);

    const QDate& date() const;
    const finances::accounts::models::Amount& amount() const;

  private:
    QDate date_;
    finances::accounts::models::Amount amount_;
};

Q_DECLARE_METATYPE(SnapshotNonNumerable);
