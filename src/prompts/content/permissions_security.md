# Guided workflow: permissions & security

Covers permission schemes, issue security schemes and levels, project roles
and role actors (who's in a role), and notification schemes. Describe every
operation as a capability to search for -- e.g. "search for how to add a
user to a project role" -- never a specific `operationId`, and read the
schema `get` returns before relying on any field name in it.

Changes here directly affect who can see or do what -- confirm the intended
scope (a single project vs. a scheme shared by many projects) with the user
before applying a change, since editing a shared scheme affects every
project that uses it, not just the one the user has in mind. Search for and
call the operation that lists a scheme's associated projects before editing
it if that's not already clear.

Verify a grant or restriction actually took effect with a follow-up read
(e.g. list the role's actors again) rather than trusting a non-error
response alone.
