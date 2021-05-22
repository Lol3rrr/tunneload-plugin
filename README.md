# Tunneload-Plugin
This contains all the functionality needed to create a plugin for Tunneload

## Structure
### Tunneload-Plugin
Provides all the Core logic and API to interact with Tunneload

### Tunneload-Plugin-Macros
Provides the `request` and `response` macros to make it more ergonomic
to expose the handlers for the Requests and Responses respectively

### Strip-Prefix
This is an example Plugin that simply strips a fixed Prefix from
the given Request
