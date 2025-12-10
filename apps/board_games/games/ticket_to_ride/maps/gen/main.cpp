#include <filesystem>
#include <fstream>
#include <iostream>

#include <CLI/CLI.hpp>

#include "apps/board_games/games/ticket_to_ride/maps/cpp/map_loader.h"

#include "arc_circle.h"

constexpr static int32_t carriage_length = 30;
constexpr static int32_t carriage_width = 10;
constexpr static int32_t gap = 4;
constexpr static int32_t double_offset = 8;
constexpr static int32_t city_radius = 8;
constexpr static int32_t city_gap = city_radius + 8;

struct RouteData {
    board_games::ticket_to_ride::City start;
    board_games::ticket_to_ride::City end;
    int32_t n_carriages;
    std::vector<board_games::ticket_to_ride::Color> colors;
    bool draw_ccw;
};

std::string city_name(const board_games::ticket_to_ride::City& city) {
    std::string name = city.name();
    std::transform(name.begin(), name.end(), name.begin(), [](unsigned char c) -> unsigned char {
        if (c == ' ')
            return '-';
        else
            return std::tolower(c);
    });
    return name;
}

// Normalize angle to [-pi, pi)
static double normalize_ang(double a) {
    while (a <= -M_PI)
        a += 2 * M_PI;
    while (a > M_PI)
        a -= 2 * M_PI;
    return a;
}

std::string draw_arc(const Vec2& center, const double& radius, const std::string& color, const double& ang_start,
                     const double& ang_end, int segments = 10, int stroke_width = 1) {
    std::ostringstream os;
    os << "<path stroke='" << color << "' stroke-width='" << stroke_width << "' fill='transparent' d='M";
    for (int i = 0; i < (segments + 1); i++) {
        double ang = ang_start + i * (ang_end - ang_start) / segments;
        double x = center.x + radius * std::cos(ang);
        double y = center.y + radius * std::sin(ang);
        os << x << " " << y << " ";
    }
    os << "'/>\n";
    return os.str();
}

std::string color(const board_games::ticket_to_ride::Color color) {
    switch (color) {
    case board_games::ticket_to_ride::COLOR_UNKNOWN:
        return "grey";
    case board_games::ticket_to_ride::COLOR_BLUE:
        return "blue";
    case board_games::ticket_to_ride::COLOR_RED:
        return "red";
    case board_games::ticket_to_ride::COLOR_GREEN:
        return "green";
    case board_games::ticket_to_ride::COLOR_YELLOW:
        return "yellow";
    case board_games::ticket_to_ride::COLOR_BLACK:
        return "black";
    case board_games::ticket_to_ride::COLOR_WHITE:
        return "white";
    case board_games::ticket_to_ride::COLOR_ORANGE:
        return "orange";
    // case board_games::ticket_to_ride::COLOR_PURPLE:
    case board_games::ticket_to_ride::COLOR_PINK:
        return "hotpink";
    case board_games::ticket_to_ride::Color_INT_MIN_SENTINEL_DO_NOT_USE_:
    case board_games::ticket_to_ride::Color_INT_MAX_SENTINEL_DO_NOT_USE_:
        return "grey";
    }
}

std::string make_route_svg(const RouteData& route, bool draw_helpers) {
    // We need to compute the arc with a given length between two points
    double arc_length =
        static_cast<double>((route.n_carriages * carriage_length) + ((route.n_carriages - 1) * gap) + (2 * city_gap));
    double hypot = std::hypot(route.end.pos_x() - route.start.pos_x(), route.end.pos_y() - route.start.pos_y());
    arc_length = std::max(arc_length, hypot);
    auto solution = arc_circle(
        Vec2{.x = static_cast<double>(route.start.pos_x()), .y = static_cast<double>(route.start.pos_y())},
        Vec2{.x = static_cast<double>(route.end.pos_x()), .y = static_cast<double>(route.end.pos_y())}, arc_length);
    if (!solution) {
        std::cerr << "There is no solution for the route from " << route.start.name() << " to " << route.end.name()
                  << std::endl;
        std::cerr << " arc lenght: " << arc_length << std::endl;

        int32_t dx = route.end.pos_x() - route.start.pos_x();
        int32_t dy = route.end.pos_y() - route.start.pos_y();
        double c = std::hypot(dx, dy);
        std::cerr << " chord lenght: " << c << std::endl;
        return "";
    }

    // TODO: Agree on some logic to choose one solution over the other.
    auto center = route.draw_ccw ? solution->centers[1] : solution->centers[0];
    double radius = solution->radius;

    // Angles of points A and B relative to center
    double ang_start = std::atan2(route.start.pos_y() - center.y, route.start.pos_x() - center.x);
    double ang_end = std::atan2(route.end.pos_y() - center.y, route.end.pos_x() - center.x);

    // We need to pick the sign (direction) so that swept absolute angle equals theta = s/R
    double theta = arc_length / radius; // positive
    // Compute normalized difference from A to B going CCW
    double diff = normalize_ang(ang_end - ang_start);

    // Prefer direction (sign) such that abs(swept) == theta (or as close as possible)
    // Two possibilities: sweep = +theta or sweep = -theta. We'll choose sign sgn such that
    // the wrapped delta is closest to sgn*theta.
    double sgn = (std::abs(normalize_ang(diff - theta)) < std::abs(normalize_ang(diff + theta))) ? +1.0 : -1.0;

    std::ostringstream os;
    os << "\n";
    os << "<g id=\"r-" << city_name(route.start) << "-" << city_name(route.end) << "\">\n";

    // HELPERS
    if (draw_helpers) {
        // - draw circle for reference
        os << "  <circle cx='" << center.x << "' cy='" << center.y << "' r='" << radius
           << "' fill='none' stroke='red'/>\n";
        // - draw chord for reference
        os << "  <line x1='" << route.start.pos_x() << "' y1='" << route.start.pos_y() << "' x2='" << route.end.pos_x()
           << "' y2='" << route.end.pos_y() << "' stroke='black'/>\n";

        // - draw city gap (start and end)
        os << draw_arc(center, radius, "green", ang_start, ang_start + sgn * (city_gap / radius), 6, 1);
        os << draw_arc(center, radius, "red", ang_start + sgn * theta - sgn * (city_gap / radius),
                       ang_start + sgn * theta, 6, 1);
    }

    // CARRIAGES
    auto draw_carriages = [&os, &draw_helpers](Vec2 _center, double _radius, double _ang_start, double _ang_end,
                                               double _sgn, int n_carriages, std::string color) {
        if (draw_helpers) {
            // - draw circle for carriages
            os << draw_arc(_center, _radius, "black", _ang_start, _ang_end, 10, 2);
        }

        // - draw carriages
        double total_length = std::abs(_ang_end - _ang_start) * _radius;
        double _carriage_length = (total_length - (n_carriages - 1) * gap) / n_carriages;
        double _carriage_ang = _carriage_length / _radius;
        double _gap_ang = gap / _radius;

        for (int i = 0; i < n_carriages; i++) {
            double ang = _ang_start + _sgn * ((_carriage_ang / 2.0) + (i * _carriage_ang) + (i * _gap_ang));
            double x = _center.x + _radius * std::cos(ang);
            double y = _center.y + _radius * std::sin(ang);

            double angdeg = ang * 180.0 / M_PI; // degrees for SVG

            os << "<circle r='5' fill='red' cx='" << x << "' cy='" << y << "' />\n";

            // Transform order: translate to pos, rotate(angle), then translate by -w/2,-h/2 to place centered
            os << "<rect width='" << carriage_width << "' height='" << _carriage_length << "' ";
            os << " fill='" << color << "' stroke='black' stroke-width='0.4'";
            os << " transform='";
            os << "translate(" << x << "," << y << ") ";
            os << "rotate(" << angdeg << ") ";
            os << "translate(" << -carriage_width / 2.0 << "," << -_carriage_length / 2.0 << ")'";
            os << "/>\n";
        }
    };

    if (route.colors.size() == 1) {
        draw_carriages(center, radius, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages,
                       color(route.colors[0]));
    } else if (route.colors.size() == 2) {
        draw_carriages(center, radius + double_offset, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages,
                       color(route.colors[0]));
        draw_carriages(center, radius - double_offset, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages,
                       color(route.colors[1]));
    }

    os << "</g>\n";

    return os.str();
}

int main(int argc, char** argv) {
    CLI::App app{"Generate SVG files for the TicketToRide maps"};
    argv = app.ensure_utf8(argv);

    std::filesystem::path input_textproto, output;
    bool draw_helpers = false;
    bool add_background = true;
    app.add_option("--textproto", input_textproto, "Input textproto file")->required();
    app.add_option("--output", output, "Output file")->required();
    app.add_option("--draw_helpers", draw_helpers, "Add helpers to the SVG output");
    app.add_option("--add_background", add_background, "Add background image");

    CLI11_PARSE(app, argc, argv);

    auto map_data_expected = board_games::ticket_to_ride::load_map_data(input_textproto);
    if (!map_data_expected) {
        std::cerr << "Abort. " << map_data_expected.error() << std::endl;
        return -1;
    }
    board_games::ticket_to_ride::MapData map_data = std::move(map_data_expected.value());

    std::ofstream os(output, std::ios::out | std::ios::binary);
    if (!os) {
        throw std::runtime_error("Cannot open file: " + output.string());
    }

    os << "<svg width=\"" << map_data.size_x() << "\" height=\"" << map_data.size_y()
       << "\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n";

    if (add_background) {
        // Background image
        os << "  <image href=\"https://i.imgur.com/3USktsR.jpeg\" width=\"" << map_data.size_x() << "\" height=\""
           << map_data.size_y() << "\"/>\n";
    }

    // Reusable elements
    os << "  <defs>\n";
    //  - city-point
    os << "    <circle r='" << city_radius << "' fill='black' stroke-width='1' stroke='black' id='city-point'/>\n";
    //  - carriage
    os << "    <g id=\"carriage\">\n";
    os << "      <rect x=\"0\" y=\"0\" width=\"" << carriage_length << "\" height=\"" << carriage_width << "\"/>\n";
    os << "    </g>\n";

    os << "  </defs>\n";

    // Cities
    std::map<int32_t, board_games::ticket_to_ride::City> cities_pos;
    os << "<!--Cities-->\n";
    for (const auto& city : map_data.cities()) {
        os << "  <use id=\"" << city_name(city) << "\" href=\"#city-point\" x=\"" << city.pos_x() << "\" y=\""
           << city.pos_y() << "\"/>\n";

        // We have tests to validate the input data
        cities_pos[city.id()] = city;
    }

    // Routes
    // - collect RouteData (// FIXME: We can improve this)
    std::map<std::pair<int32_t, int32_t>, RouteData> routes;
    {
        for (const auto& route : map_data.routes()) {
            // 'key' is independent on the order of the cities
            auto city1 = std::min(route.city1(), route.city2());
            auto city2 = std::max(route.city1(), route.city2());
            std::pair<int32_t, int32_t> key = std::make_pair(city1, city2);

            // ...but the route has a direction
            routes[key].start = cities_pos[route.city1()];
            routes[key].end = cities_pos[route.city2()];
            routes[key].n_carriages = route.length();
            routes[key].colors.push_back(route.color());
            routes[key].draw_ccw = route.draw_ccw();
        }
    }
    os << "<!--Routes-->\n";
    for (const auto& [_, route] : routes) {
        std::string svg = make_route_svg(route, draw_helpers);
        os << svg;
    }

    // Close tag
    os << "</svg>\n";

    return 0;
}
