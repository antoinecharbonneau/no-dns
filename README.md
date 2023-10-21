# No DNS (No DistractioNS)

No DNS is a reasonably performant multithreaded 
DNS proxy server written in Rust that helps you fight against distractions.

## Features

* Multithreading to process multiple requests simultaneously.
* Blocklist with elementary matching
* Caching for lower latency on common requests

## Using No DNS

To use No DNS with a custom blocklist, run it with the `f` flag to specify the blocklist, the `b` flag to select the binding address, and the `u` flag to choose your upstream dns server.

    $ no-dns -f blocklist.txt -u 8.8.8.8

No DNS requires to be able to bind to the port 53 in UDP. If your port is already bound by a service, consider using a container.

If a requested domain is contained in the blocklist, the proxy will respond as if it doesn't know the domain. Otherwise, the proxy will forward the request to the upstream server, caching the result.

### Blocklist

Currently, the format to make a blocklist is one domain per row of a text file.

    example.com
    google.com
    bing.com

To block every subdomain to a domain, use an asterisk :

    *.google.com
    *.io

## Planned features

* Migrating to tokio runtime
* More flexible blocklist (i.e. blocking between time periods, blocking for specific IPs)
* TCP fallback
* Better logging
* Metrics (requests rate, cache hit, request blocked)
* Multithreading to process sub-elements of a given request.
