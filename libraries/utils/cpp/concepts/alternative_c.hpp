
#include <type_traits>
#include <variant>

// Credit:
// https://stackoverflow.com/questions/78212352/c-concept-to-match-an-alternative-of-a-stdvariant-but-that-also-works-with

namespace utils::concepts {

    namespace detail {

        template <class... Ts> std::variant<Ts...> to_variant(const std::variant<Ts...>&);

        template <class TAlternative, class TVariant> inline constexpr bool is_alternative_v = false;

        template <class TAlternative, class... Ts>
        inline constexpr bool is_alternative_v<TAlternative, std::variant<Ts...>> =
            (... or std::is_same_v<TAlternative, Ts>);

    } // namespace detail

    /// Checks if `TAlternative` is included inside the types in the `TVariant` variant.
    template <class TAlternative, class TVariant>
    concept alternative_in_variant =
        requires(TVariant var) { requires detail::is_alternative_v<TAlternative, decltype(detail::to_variant(var))>; };

    /// Checks if `TAlternative` is included inside the `Ts...` types.
    template <class TAlternative, class... Ts>
    concept alternative_in_pack = (... or std::is_same_v<TAlternative, Ts>);

} // namespace utils::concepts
