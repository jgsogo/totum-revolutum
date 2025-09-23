#pragma once

#include <spdlog/spdlog.h>

namespace spdlog::utils {

    struct level_raii {
        level_raii(spdlog::level::level_enum l) {
            old_level = spdlog::get_level();
            spdlog::set_level(l);
        }

        ~level_raii() { spdlog::set_level(old_level); }

        spdlog::level::level_enum old_level;
    };

    template <typename R> R with_level(spdlog::level::level_enum l, std::function<R()> f) {
        level_raii _raii(l);
        return f();
    }

} // namespace spdlog::utils
