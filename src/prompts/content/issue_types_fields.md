# Guided workflow: issue types & fields

Covers issue types, issue type schemes and their project/issue-type
associations, fields, custom fields, and screens/screen tabs (which fields
appear on which screen). Describe every operation as a capability to search
for -- e.g. "search for how to associate an issue type with a project's
issue type scheme" -- never a specific `operationId`, and read the schema
`get` returns before relying on any field name in it.

## Real gotchas

- **Which issue types exist for a project** is governed by that project's
  issue type scheme association, not a global list -- search for the
  project's issue type scheme before assuming an issue type is available
  there.
- **Which fields are required or even visible** for a given project +
  issue type combination depends on the screen scheme and field
  configuration in play, not just whether the field exists globally. This
  is exactly what `jira-issues` searches for before creating an
  issue -- reuse that pattern here rather than assuming a custom field
  applies everywhere it's defined.
- **Custom field creation and custom field *usage*** are different
  operations: creating a custom field makes it exist; a separate
  screen/field-configuration association step is what actually makes it
  show up for a given project and issue type.

## Typical flow

1. Confirm the project (and issue type, if relevant) the user is working
   in.
2. Search for and call the operation to read the current issue type
   scheme, field configuration, or screen -- don't assume the current
   state.
3. Make the requested change, then verify it with a follow-up read.

For workflow-status behavior once an issue type exists, see
`jira-workflows-statuses`.
