#include "utils.h"

namespace utils {

    tl::expected<finances::accounts::models::Amount, std::string>
    qstring_to_amount(const QString& input, finances::accounts::models::Ccy ccy) {
        // TODO: Different Ccy might have different parse formats

        static const dec::decimal_format SPANISH_DECIMAL_FORMAT{','}; // FIXME: Get this from Qt locale
        finances::accounts::models::Amount::InnerType output;

        std::istringstream is(input.toStdString());
        bool success = dec::fromStream(is, SPANISH_DECIMAL_FORMAT, output);
        if (!success) {
            return tl::unexpected{std::format("Failed to convert {} to decimal type", input.toStdString())};
        }
        return {finances::accounts::models::Amount{std::move(output)}};
    }

} // namespace utils
