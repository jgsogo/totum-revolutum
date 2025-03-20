"""Generate a script to append a host to /etc/hosts file"""

def _append_hosts_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.name)

    host = "{}       {}".format(ctx.attr.ip, ctx.attr.host)
    ctx.actions.expand_template(
        template = ctx.file._template,
        output = output,
        substitutions = {
            "%HOST%": host,
        },
    )
    return [
        DefaultInfo(files = depset([output])),
    ]

append_hosts = rule(
    implementation = _append_hosts_impl,
    attrs = {
        "host": attr.string(mandatory = True),
        "ip": attr.string(default = "127.0.0.1"),
        "_template": attr.label(
            default = Label("//bazel/scripts/hosts:append_hosts.tpl.sh"),
            allow_single_file = True,
        ),
    },
    doc = "A script to append a host to /etc/hosts file",
)
