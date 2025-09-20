#pragma once

#include "libraries/utils/cpp/string_type.hpp"

namespace finances::accounts::models {
    using Country = utils::StringType<class CountryTag>;

    using namespace std::literals::string_view_literals;
    constexpr static Country Spain{"SPA"sv};
    constexpr static Country USA{"USA"sv};
    constexpr static Country Netherlands{"NL"sv};
} // namespace finances::accounts::models
