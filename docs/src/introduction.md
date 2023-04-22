# Introduction

## Features
### Secure by default.
Privileged access to logs and servers must be explicity provided upon authorization.

### Asynchronous reads.
Reads of files are spread across servers/files aschronously to improve file read and aggregation speed.

## Endpoints
### **POST** `/authorize`
#### Body
**path**: The files inside of /var/log the user needs to access. By default, a user has no access to any files.

**servers**: The servers the user will aggregate log data from. By default, a user has no access to any servers.
