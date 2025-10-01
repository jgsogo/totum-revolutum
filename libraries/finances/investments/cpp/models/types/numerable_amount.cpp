#include "numerable_amount.h"

namespace finances::investments::models {

    ::finances::accounts::models::Amount NumerableAmount::amount() const {
        auto r = quantity.value * unit_value.value;
        return ::finances::accounts::models::Amount{r};
    }

} // namespace finances::investments::models
