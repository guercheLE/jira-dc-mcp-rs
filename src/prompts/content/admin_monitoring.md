# Guided workflow: admin & monitoring

A thin pointer, not a multi-step flow: for cluster status, JMX/other
monitoring signals, reindex and index-snapshot operations, server info,
application properties, and dashboards, search for the specific read-only
or admin signal the user is after (e.g. "search for how to check cluster
node status", "search for how to trigger a reindex") and call the matching
operation directly -- read the schema `get` returns before relying on any
field name in it, since these operations skew toward admin-only and can
differ across the supported API versions. If the request is a broad
"what's going on" question rather than one specific signal, and your
environment provides a way to run a sub-task in its own context, delegate
the exploration and have it return only the few signals that actually
matter rather than every listing it touched along the way.
