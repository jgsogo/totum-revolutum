#include "colors.h"

svg::Color color(const board_games::ticket_to_ride::Color color) {
    switch (color) {
    case board_games::ticket_to_ride::COLOR_UNKNOWN:
        return svg::Color::GREY;
    case board_games::ticket_to_ride::COLOR_BLUE:
        return svg::Color::BLUE;
    case board_games::ticket_to_ride::COLOR_RED:
        return svg::Color::RED;
    case board_games::ticket_to_ride::COLOR_GREEN:
        return svg::Color::GREEN;
    case board_games::ticket_to_ride::COLOR_YELLOW:
        return svg::Color::YELLOW;
    case board_games::ticket_to_ride::COLOR_BLACK:
        return svg::Color::BLACK;
    case board_games::ticket_to_ride::COLOR_WHITE:
        return svg::Color::WHITE;
    case board_games::ticket_to_ride::COLOR_ORANGE:
        return svg::Color::ORANGE;
    // case board_games::ticket_to_ride::COLOR_PURPLE:
    case board_games::ticket_to_ride::COLOR_PINK:
        return svg::Color::HOTPINK;
    case board_games::ticket_to_ride::Color_INT_MIN_SENTINEL_DO_NOT_USE_:
    case board_games::ticket_to_ride::Color_INT_MAX_SENTINEL_DO_NOT_USE_:
        return svg::Color::GREY;
    }
}
