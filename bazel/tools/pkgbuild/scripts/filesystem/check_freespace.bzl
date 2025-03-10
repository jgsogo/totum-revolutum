"""Module providing some sh scripts to run bash checks"""

def _check_freespace_impl(ctx):
    output = ctx.actions.declare_file("check-free-space")

    content = """
# Ensure at least {mb}MB free space in the install folder
log_debug "Check free space"
FREE_SPACE=$(df -m ${{INSTALL_FOLDER}} | tail -1 | awk '{{print $4}}')
if [ "$FREE_SPACE" -lt {mb} ]; then
    error "Not enough disk space in ${{INSTALL_FOLDER}} (needs at least {mb}MB)."
fi
""".format(mb = ctx.attr.megabytes)

    ctx.actions.write(
        output = output,
        content = content,
    )

    return [
        DefaultInfo(files = depset([output])),
    ]

check_freespace = rule(
    implementation = _check_freespace_impl,
    attrs = {
        "megabytes": attr.string(doc = "Memory required (in megabytes)"),
    },
    doc = "Check free space",
)
