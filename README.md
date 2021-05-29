# Tunneload-Plugin
This contains all the functionality needed to create a plugin for Tunneload

## Structure
### Tunneload-Plugin
Provides all the Core logic and API to interact with Tunneload

### Tunneload-Plugin-Macros
Provides the Macros to make it more ergonomic to expose the Plugin and handle
all the "glue"-Code between Tunneload and the Plugin.

### Strip-Prefix
This is an example Plugin that simply strips a given Prefix from
the given Requests

### Set-Header
This is an example Plugin that demonstrates and tests the different
capabilietes of the Handlers, like returning Error-Responses
