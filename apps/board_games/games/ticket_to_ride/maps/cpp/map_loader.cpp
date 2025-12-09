#include "map_loader.h"

#include <fstream>

#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/io/zero_copy_stream_impl.h>
#include <google/protobuf/text_format.h>

namespace board_games::ticket_to_ride {

    utils::ExpectedType<MapData, ErrorLoadingFile, ErrorTextProtoParse>
    load_map_data(const std::filesystem::path& path) {
        std::ifstream stream(path.c_str(), std::ios::in | std::ios::binary);
        if (!stream.is_open()) {
            return tl::unexpected{ErrorLoadingFile{std::format("Cannot open the file '{}'", path.c_str())}};
        }

        google::protobuf::io::IstreamInputStream zero_copy_input(&stream);

        MapData data;

        if (!google::protobuf::TextFormat::Parse(&zero_copy_input, &data)) {
            return tl::unexpected{ErrorTextProtoParse{std::format("Failed to parse textproto '{}'", path.c_str())}};
        }

        return {data}; // RVO
    }

    utils::ExpectedType<MapData, ErrorLoadingFile, ErrorBinaryProtoParse>
    load_map_data_binary(const std::filesystem::path& path) {
        std::ifstream stream(path.c_str(), std::ios::in | std::ios::binary);
        if (!stream.is_open()) {
            return tl::unexpected{ErrorLoadingFile{std::format("Cannot open the file '{}'", path.c_str())}};
        }

        MapData data;
        if (!data.ParseFromIstream(&stream)) {
            return tl::unexpected{
                ErrorBinaryProtoParse{std::format("Failed to parse binary protobuf file '{}'", path.c_str())}};
        }

        return data; // RVO
    }

} // namespace board_games::ticket_to_ride
