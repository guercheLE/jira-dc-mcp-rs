# Guided workflow: users & groups

Covers user and group lifecycle, application roles, the caller's own
`myself`/preferences, session/websudo authentication state, and password
policy. Describe every operation as a capability to search for -- e.g.
"search for how to add a user to a group" -- never a specific `operationId`,
and read the schema `get` returns before relying on any field name in it.

**Real gotcha:** deleting or anonymizing a user is not always instant --
search for how user deletion/anonymization works for this version before
telling the user it's done; some versions run it as an async job that needs
a follow-up status check rather than completing synchronously in the
delete/anonymize call's own response.

For granting a user or group access to a specific project or scheme (as
opposed to managing the user/group itself), see
`jira-permissions-security`.
