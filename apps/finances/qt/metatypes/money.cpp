#include "money.h"

// QtMoney::QtMoney(const finances::accounts::models::Money& money) {}

// finances::accounts::models::Money QtMoney::money() const {
//     finances::accounts::models::Amount::InnerType amount{0};
//     finances::accounts::models::Money ret{finances::accounts::models::Amount{amount},
//     finances::accounts::models::EUR}; return ret;
// }

QDebug operator<<(QDebug dbg, const QtMoney& money) {
    QDebugStateSaver saver(dbg);

    dbg.nospace() << static_cast<std::string>(money);

    return dbg;
}
