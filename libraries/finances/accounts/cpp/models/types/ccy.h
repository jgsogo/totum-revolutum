#pragma once

#include "libraries/utils/cpp/string_type.hpp"

namespace finances::accounts::models {
    using Ccy = utils::StringType<class CCyTag>;

    using namespace std::literals::string_view_literals;
    constexpr static Ccy EUR{"EUR"sv};
    constexpr static Ccy USD{"USD"sv};
} // namespace finances::accounts::models
