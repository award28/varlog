# API Specification

The following yaml provides an overview of the API Specification for the
Sidecar REST API's V1 Endpoints. Additionally, you can [download the Postman collection](../assets/Varlog.postman_collection.json).

```yaml
openapi: 3.0.0
info:
  title: Varlog
  version: 1.0.0
servers:
  - url: http://localhost:8080
  - url: http://localhost:8888
components:
  securitySchemes:
    jwtAuth:
      type: http
      scheme: jwt
paths:
  /v1/auth/register:
    post:
      tags:
        - default
      summary: Authorization Registration
      requestBody:
        content:
          application/json:
            schema:
              type: object
              example:
                paths:
                  - .*
                servers:
                  - .*
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /v1/logs:
    get:
      tags:
        - default
      summary: Logs
      security:
        - jwtAuth: []
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /v1/servers:
    get:
      tags:
        - default
      summary: Servers
      security:
        - jwtAuth: []
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /v1/servers/logs:
    get:
      tags:
        - default
      summary: Servers Logs
      security:
        - jwtAuth: []
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /v1/servers/logs/{filename}:
    get:
      tags:
        - default
      summary: Read Servers Log
      security:
        - jwtAuth: []
      parameters:
        - name: filename
          in: path
          schema:
            type: string
          required: true
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /v1/logs/logged.log:
    get:
      tags:
        - default
      summary: Read Log
      security:
        - jwtAuth: []
      parameters:
        - name: filename
          in: query
          schema:
            type: string
          example: logged.log
        - name: take
          in: query
          schema:
            type: integer
          example: '100'
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /register:
    post:
      tags:
        - default
      summary: Register
      requestBody:
        content:
          application/json:
            schema:
              type: object
              example:
                hostname: hello
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
  /registered:
    get:
      tags:
        - default
      summary: Registered
      responses:
        '200':
          description: Successful response
          content:
            application/json: {}
```

