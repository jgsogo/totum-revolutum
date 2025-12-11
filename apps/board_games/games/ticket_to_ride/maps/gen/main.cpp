#include <filesystem>
#include <fstream>
#include <iostream>

#include <CLI/CLI.hpp>

#include "libraries/utils/cpp/math/geometry/2d/angle.hpp"
#include "libraries/utils/cpp/math/geometry/2d/circle_for_arc.hpp"
#include "libraries/utils/cpp/svg/circle.h"
#include "libraries/utils/cpp/svg/image.h"
#include "libraries/utils/cpp/svg/line.h"
#include "libraries/utils/cpp/svg/path.h"
#include "libraries/utils/cpp/svg/rect.h"

#include "apps/board_games/games/ticket_to_ride/maps/cpp/map_loader.h"

#include "colors.h"
#include "route_data.h"

constexpr static int32_t carriage_length = 40;
constexpr static int32_t carriage_width = 10;
constexpr static int32_t gap = 4;
constexpr static int32_t double_offset = 8;
constexpr static int32_t city_radius = 8;
constexpr static int32_t city_gap = city_radius + 8;

using namespace utils::math::g2d;

void make_route_svg(svg::SVGGroup& route_group, const RouteData& route, bool draw_helpers) {
    Point<double> route_start = static_cast<Point<double>>(route.start.pos);
    Point<double> route_end = static_cast<Point<double>>(route.end.pos);

    // We need to compute the arc with a given length between two points
    double arc_length =
        static_cast<double>((route.n_carriages * carriage_length) + ((route.n_carriages - 1) * gap) + (2 * city_gap));
    double chord_length = (route_start - route_end).hypot();
    arc_length = std::max(arc_length, chord_length);
    auto solution_opt = circle_for_arc(route_start, route_end, arc_length);
    if (!solution_opt) {
        std::cerr << "There is no solution for the route from " << route.start.name << " to " << route.end.name
                  << std::endl;
        std::cerr << " arc lenght: " << arc_length << std::endl;
        std::cerr << " chord lenght: " << chord_length << std::endl;
        return;
    }

    // TODO: Agree on some logic to choose one solution over the other.
    auto solution = route.draw_ccw ? solution_opt->second : solution_opt->first;
    auto center = solution.center;
    double radius = solution.radius;

    // Angles of points A and B relative to center
    double ang_start = atan2(route_start - center);
    double ang_end = atan2(route_end - center);

    // We need to pick the sign (direction) so that swept absolute angle equals theta = s/R
    double theta = arc_length / radius; // positive
    // Compute normalized difference from A to B going CCW
    double diff = normalize_ang(ang_end - ang_start);

    // Prefer direction (sign) such that abs(swept) == theta (or as close as possible)
    // Two possibilities: sweep = +theta or sweep = -theta. We'll choose sign sgn such that
    // the wrapped delta is closest to sgn*theta.
    double sgn = (std::abs(normalize_ang(diff - theta)) < std::abs(normalize_ang(diff + theta))) ? +1.0 : -1.0;

    // HELPERS
    if (draw_helpers) {
        // - draw circle for reference
        svg::Circle& circle_ref = route_group.add_from<svg::Circle>(solution);
        circle_ref.fill = svg::Color::NONE;
        circle_ref.stroke = svg::Color::RED;

        // - draw chord for reference
        svg::Line& chord_ref = route_group.add<svg::Line>();
        chord_ref.stroke = svg::Color::BLACK;
        chord_ref.start = route.start.pos;
        chord_ref.end = route.end.pos;

        // - draw city gap (start and end)
        svg::Path& city_gap_start =
            route_group.add_from<svg::Path>(solution, ang_start, ang_start + sgn * (city_gap / radius), 6);
        city_gap_start.stroke = svg::Color::GREEN;
        city_gap_start.stroke_width = 1;
        city_gap_start.fill = svg::Color::TRANSPARENT;

        svg::Path& city_gap_end = route_group.add_from<svg::Path>(
            solution, ang_start + sgn * theta - sgn * (city_gap / radius), ang_start + sgn * theta, 6);
        city_gap_end.stroke = svg::Color::RED;
        city_gap_end.stroke_width = 1;
        city_gap_end.fill = svg::Color::TRANSPARENT;
    }

    // CARRIAGES
    Point<double> carriage_size{.x = carriage_width, .y = carriage_length};
    auto draw_carriages = [&route_group, &draw_helpers, &carriage_size](const Circunference<double>& _circunference,
                                                                        double _ang_start, double _ang_end, double _sgn,
                                                                        int n_carriages, svg::Color color) {
        if (draw_helpers) {
            // - draw circle for carriages
            svg::Path& circle_carriages = route_group.add_from<svg::Path>(_circunference, _ang_start, _ang_end, 10);
            circle_carriages.stroke = svg::Color::BLACK;
            circle_carriages.stroke_width = 2;
            circle_carriages.fill = svg::Color::TRANSPARENT;
        }

        // - draw carriages
        double total_length = std::abs(_ang_end - _ang_start) * _circunference.radius;
        double _gap = n_carriages > 1 ? (total_length - (n_carriages * carriage_length)) / (n_carriages - 1) : 0;
        double _carriage_ang = carriage_length / _circunference.radius;
        double _gap_ang = _gap / _circunference.radius;

        for (int i = 0; i < n_carriages; i++) {
            double ang = _ang_start + _sgn * ((_carriage_ang / 2.0) + (i * _carriage_ang) + (i * _gap_ang));
            Point<double> circunference_radius{.x = _circunference.radius * std::cos(ang),
                                               .y = _circunference.radius * std::sin(ang)};
            Point<double> xy = _circunference.center + circunference_radius;

            double angdeg = ang * 180.0 / M_PI; // degrees for SVG

            if (draw_helpers) {
                svg::Circle& carriage_circle = route_group.add<svg::Circle>();
                carriage_circle.fill = svg::Color::RED;
                carriage_circle.center = static_cast<Point<int>>(xy);
                carriage_circle.radius = 5;
            }

            // Transform order: translate to pos, rotate(angle), then translate by -w/2,-h/2 to place centered
            svg::Rect& rect = route_group.add<svg::Rect>();
            rect.size = static_cast<Point<int>>(carriage_size);
            rect.fill = color;
            rect.stroke = svg::Color::BLACK;
            rect.stroke_width = 0.4;
            rect.transformation.emplace_back(
                std::make_unique<svg::Translate>(svg::Translate{static_cast<Point<int>>(xy)}));
            rect.transformation.emplace_back(std::make_unique<svg::Rotate>(svg::Rotate{static_cast<float>(angdeg)}));
            rect.transformation.emplace_back(std::make_unique<svg::Translate>(svg::Translate{-carriage_size / 2.0}));
        }
    };

    if (route.colors.size() == 1) {
        draw_carriages(solution, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages, route.colors[0]);
    } else if (route.colors.size() == 2) {
        auto circ_external = Circunference{.center = solution.center, .radius = solution.radius + double_offset};
        draw_carriages(circ_external, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages, route.colors[0]);

        auto circ_internal = Circunference{.center = solution.center, .radius = solution.radius - double_offset};
        draw_carriages(circ_internal, ang_start + sgn * (city_gap / radius),
                       ang_start + sgn * theta - sgn * (city_gap / radius), sgn, route.n_carriages, route.colors[1]);
    }
}

int main(int argc, char** argv) {
    CLI::App app{"Generate SVG files for the TicketToRide maps"};
    argv = app.ensure_utf8(argv);

    std::filesystem::path input_textproto, output;
    bool draw_helpers = false;
    bool add_background = false;
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

    svg::SVGDoc doc;
    doc.size = Point<int>{.x = map_data.size_x(), .y = map_data.size_y()};

    if (add_background) {
        // Background image
        svg::Image& background = doc.add<svg::Image>();
        background.href = "https://i.imgur.com/3USktsR.jpeg";
        background.size = Point<int>{.x = map_data.size_x(), .y = map_data.size_y()};
    }

    // Reusable elements
    svg::SVGDefs& svg_defs = doc.add<svg::SVGDefs>();
    {
        //  - city-point
        auto& city_point = svg_defs.add<svg::Circle>();
        city_point.radius = city_radius;
        city_point.fill = svg::Color::BLACK;
        city_point.stroke_width = 1;
        city_point.stroke = svg::Color::BLACK;
        city_point.id = "city-point";

        //  - carriage
        auto& carriage_group = svg_defs.add<svg::SVGGroup>();
        carriage_group.id = "carriage";
        auto& carriage_rect = carriage_group.add<svg::Rect>();
        carriage_rect.size = Point<int>{.x = carriage_length, .y = carriage_width};
    }

    // Cities
    std::map<int32_t, City> cities;
    for (const auto& proto_city : map_data.cities()) {
        auto city = City::from(proto_city);
        cities[city.idx] = city; // No duplicated, there are tests!

        auto& svg_city = doc.add<svg::SVGUse>("#city-point");
        svg_city.id = city.id;
        svg_city.pos = city.pos;
    }

    // Routes
    // - collect RouteData (// FIXME: We can improve this)
    std::map<std::pair<int32_t, int32_t>, RouteData> routes;
    {
        for (const auto& proto_route : map_data.routes()) {
            // 'key' is independent on the order of the cities
            auto city1 = std::min(proto_route.city1(), proto_route.city2());
            auto city2 = std::max(proto_route.city1(), proto_route.city2());
            std::pair<int32_t, int32_t> key = std::make_pair(city1, city2);

            // ...but the route has a direction
            routes[key].start = cities[proto_route.city1()];
            routes[key].end = cities[proto_route.city2()];
            routes[key].n_carriages = proto_route.length();
            routes[key].colors.push_back(color(proto_route.color()));
            routes[key].draw_ccw = proto_route.draw_ccw();
        }
    }
    for (const auto& [_, route] : routes) {
        svg::SVGGroup& route_group = doc.add<svg::SVGGroup>();
        route_group.id = std::format("r-{}-{}", route.start.id, route.end.id);

        make_route_svg(route_group, route, draw_helpers);
    }

    std::ofstream os(output, std::ios::out | std::ios::binary);
    if (!os) {
        throw std::runtime_error("Cannot open file: " + output.string());
    }
    os << doc;

    return 0;
}
