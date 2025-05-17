# opnsense-unbound-webhook

A Kubernetes ExternalDNS webhook written in Rust for managing Unbound DNS
overrides in OPNsense.

This project provides a webhook backend for
[ExternalDNS](https://github.com/kubernetes-sigs/external-dns) that integrates
with [OPNsense](https://opnsense.org/)'s Unbound DNS service, enabling automated
management of DNS overrides from Kubernetes.

# Overview

`opnsense-unbound-webhook` facilitates Kubernetes-native DNS configuration by
bridging ExternalDNS and OPNsense. It follows the ExternalDNS [webhook
provider specification](https://kubernetes-sigs.github.io/external-dns/latest/docs/tutorials/webhook-provider/).

## Features

- Fully implements the ExternalDNS webhook schema.
- Safe and idempotent with authenticated access to OPNsense. 
- Supports creation and deletion of Unbound DNS overrides and host aliases.

# Architecture

This repository consumes the `opnsense` Unbound APIs and exposes them in a
well-typed rust crate called `opnsense`. While this currently only exposes the
necessary Unbound APIs, it can easily be extended to support more of the
`opnsense` API.

# Usage

> This project is under active development. Interfaces and functionality are
> subject to change.

TODO

<!-- ### Requirements -->

<!-- - A running Kubernetes cluster -->
<!-- - ExternalDNS configured to use a webhook provider -->
<!-- - OPNsense with API access enabled and Unbound DNS configured -->

<!-- ### ExternalDNS Sample Configuration -->

<!-- ```yaml -->
<!-- args: -->
<!--   - --provider=webhook -->
<!--   - --webhook-provider-url=http://opnsense-unbound-webhook.default.svc.cluster.local -->
<!--   - --webhook-provider-root=/external-dns -->
<!--   - --webhook-provider-secret-name=opnsense-api-credentials -->
<!-- ``` -->
