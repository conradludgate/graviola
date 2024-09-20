#!/usr/bin/python

from os import path
import json
from io import StringIO

archs = "aarch64 x86_64".split()
impls = "aws-lc-rs dalek ring graviola rustcrypto".split()
which = {
    ("rsa2048-pkcs1-sha256-verify",): {
        "name": "RSA2048 signature verification",
        "format": lambda v: "<data value='{0}'>{0:,.5g}</data> sigs/sec".format(v),
    },
    ("rsa2048-pkcs1-sha256-sign",): {
        "name": "RSA2048 signing",
        "format": lambda v: "<data value='{0}'>{0:,.5g}</data> sigs/sec".format(v),
    },
    ("x25519-ecdh",): {
        "name": "X25519 key agreement",
        "format": lambda v: "<data value='{0}'>{0:,.5g}</data> kx/sec".format(v),
    },
    ("aes256-gcm", "16KB"): {
        "name": "AES256-GCM encryption (16KB wide)",
        "format": lambda v: "<data value='{0}'>{1:,.03g}</data> GiB/sec".format(
            v, v * 16384 / (1024 * 1024 * 1024)
        ),
    },
}

results = []

standings = ["🥇", "🥈", "🥉", " ", " "]

for arch in sorted(archs):
    for wkey, wdesc in sorted(which.items()):
        if len(wkey) > 1:
            wkey, wsize = wkey
        else:
            (wkey,) = wkey
            wsize = None

        for impl in sorted(impls):
            filename = path.join(
                "reports",
                arch,
                wkey,
                impl,
                wsize + "/new" if wsize else "new",
                "estimates.json",
            )
            if not path.exists(filename):
                continue
            data = json.load(open(filename))
            value = data["median"]["point_estimate"]

            results.append((wdesc, arch, impl, value))

tree = {}
for desc, arch, impl, value in results:
    time = value * 1e-9
    rate = 1 / time
    value = desc["format"](rate)
    tree.setdefault(arch, {}).setdefault(desc["name"], {})[impl] = (value, rate)

fragment = StringIO()
for arch, descs in sorted(tree.items()):
    print("<h2>{}</h2>".format(arch), file=fragment)
    for desc, impls in sorted(descs.items()):
        print(
            "<h3>{}</h3><table width='80%' cellspacing=25 cellpadding=10><tr>".format(
                desc
            ),
            file=fragment,
        )

        for standing, impl in zip(
            standings, sorted(impls.keys(), key=lambda b: impls[b][1], reverse=True)
        ):
            print(
                "<td class='{}'><h3>{} {}</h3>{}</td>".format(
                    impl, standing, impl, impls[impl][0]
                ),
                file=fragment,
            )

        print("</tr></table>", file=fragment)


html = open("index.html").readlines()
out = open("index.html.new", "w")

skipping = False
for line in html:
    if not skipping:
        out.write(line)

    if line == "<!-- begin headlines -->\n":
        skipping = True
        out.write(fragment.getvalue())
    if skipping and line == "<!-- end headlines -->\n":
        skipping = False
        out.write(line)
