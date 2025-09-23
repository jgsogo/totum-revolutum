#pragma once

#include <spdlog/sinks/ostream_sink.h>
#include <spdlog/spdlog.h>

// https://stackoverflow.com/questions/66473052/how-can-i-read-spdlog-output-in-a-google-test

// std::ostringstream oss;
// auto ostream_sink = std::make_shared<spdlog::sinks::ostream_sink_mt> (oss);
// auto logger = std::make_shared<spdlog::logger>("my_logger", ostream_sink);

namespace utils::catch2 {

    class CaptureSpdlogFixture {
      protected:
        std::ostringstream oss;

      public:
        CaptureSpdlogFixture() {
            auto ostream_sink = std::make_shared<spdlog::sinks::ostream_sink_mt>(oss);
            auto logger = std::make_shared<spdlog::logger>("my_logger", ostream_sink);
            spdlog::set_default_logger(logger);
        }

        void reset() { oss.str(""); }
    };

} // namespace utils::catch2
