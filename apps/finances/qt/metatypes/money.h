#pragma once

#include <QDebug>

#include "libraries/finances/accounts/cpp/models/types/money.h"

// class QtMoney {
//   public:
//     QtMoney() = default;
//     ~QtMoney() = default;
//     QtMoney(const QtMoney&) = default;
//     QtMoney& operator=(const QtMoney&) = default;

//     QtMoney(const finances::accounts::models::Money& money);

//     finances::accounts::models::Money money() const;

//   private:
// };

using QtMoney = finances::accounts::models::Money;

QDebug operator<<(QDebug dbg, const QtMoney& money);

Q_DECLARE_METATYPE(QtMoney);
