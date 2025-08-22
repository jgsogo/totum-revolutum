#pragma once

namespace data {

    // FIXME: Use strong types instead, see https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjhxti2/,
    //        and we can write some widening-variant like https://www.reddit.com/r/cpp/comments/19eqc9p/comment/kjrfl5z/
    //        and move this to a reusable taget built on top of std::expected
    enum class Error {
        InsertError,
        SelectError,
        DBError,
        NotifcationFailed,
        NotFound,
    };

} // namespace data
