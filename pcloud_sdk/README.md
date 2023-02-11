pCloud-sdk
==========

Implementation of the pCloud API in Rust... while learning the
language (help and suggestion are very welcome).

## Authentication

Only OAuth2 using an application is implemented. Using username/password
should be discouraged: inherently more dangerous and authorization is harder
to revoke, also it is not possible if two-factor authentication is enabled.

To use the library you will need to create a pCloud application and get its
`client_id` and `client_secret`. If you want to emulate username/password
access, just create an application with write permissions to every folder.
Then, store credentials in a JSON file like:

```json
{
  "client_id": "<client_id>",
  "client_secret": "<client_secret>",
  "force_reapprove": false
}
```

> **Note.-** Remember that you can always revoke access to your account for any application
from your pCloud account in `Settings >> Linked accounts >> Linked apps`.

