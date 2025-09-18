"""
"""

def get_base_name(name, version, host, arch, target_sdk):
    return "{name}_{v}_{host}_{arch}_{target_sdk}".format(
        name = name,
        arch = arch,
        host = host,
        target_sdk = target_sdk,
        v = version,
    )

def get_build_filename(name, version, host, arch, target_sdk):
    base_name = get_base_name(name, version, host, arch, target_sdk)
    return "{}.BUILD".format(base_name)
