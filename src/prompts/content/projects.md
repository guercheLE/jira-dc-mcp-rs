# Guided workflow: projects

Covers project lifecycle (create/read/update/delete), project categories,
components, and versions (releases). Describe every operation as a
capability to search for -- e.g. "search for how to create a component in a
project" -- never a specific `operationId`, and read the schema `get`
returns before relying on any field name in it.

Ask for the project key up front if the user hasn't given one; most
component/version operations are scoped to a single project and need it.

**Real gotchas:**

- **Releasing a version.** Before marking a version released, search for
  and call the operations that report its unresolved-issue and
  related-issue counts, and surface those to the user -- releasing with
  unresolved issues attached is usually not what they intend, even though
  the API permits it.
- **Deleting or merging a version.** A plain delete leaves referencing
  issues with a dangling version field unless you search for and use the
  dedicated "delete and swap" operation (reassigns issues to a replacement
  version) or the "merge versions" operation (folds one version's issues
  into another) -- ask the user which behavior they want rather than
  defaulting to a plain delete.
- **Archiving a project.** A project can be archived and later restored,
  distinct from deletion -- search for the archive/restore operations if
  the user wants a project out of the way but recoverable.

For creating and managing issue types, issue type schemes, and custom
fields available to a project, see `jira-issue-types-fields`. For
permission/security scheme assignment, see
`jira-permissions-security`. For bootstrapping a brand new project
end to end (type selection, creation, and scheme wiring), see
`jira-project-setup`.
