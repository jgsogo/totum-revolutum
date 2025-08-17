#pragma once

#include <spdlog/spdlog.h>

namespace spdlog::utils {

    template <typename R> R with_level(spdlog::level::level_enum l, std::function<R()> f) {
        spdlog::level::level_enum old_level = spdlog::get_level();
        spdlog::set_level(l);
        auto r = f();
        spdlog::set_level(old_level);
        return r;
    }

    template <> void with_level(spdlog::level::level_enum l, std::function<void()> f);

} // namespace spdlog::utils
