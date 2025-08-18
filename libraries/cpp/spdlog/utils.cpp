#include "utils.hpp"

namespace spdlog::utils {

    template <> void with_level(spdlog::level::level_enum l, std::function<void()> f) {
        spdlog::level::level_enum old_level = spdlog::get_level();
        spdlog::set_level(l);
        f();
        spdlog::set_level(old_level);
    }

} // namespace spdlog::utils
