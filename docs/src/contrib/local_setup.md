# Local Setup

In this section of the documentation, we'll review what's required to get Varlog
running on your local. For the fastest response when building the app, we recommend
the bare metal setup. However, the docker setup is a better replica of the prod
environment, and should be used to validate the integration of new features.

Regardless of which setup type you choose, there is one thing you'll need to do
for both. You'll need to create a `.env` file in the root of the project. 
This file is used to populate the expected environment variables.

```python
JWT_SIGNING_KEY="a-super-secure-signing-key"
HOSTNAME="localhost:8080"
REGISTRY_URL="http://localhost:8888"
```

Optionally, you can create some fake data using the
[fakedata](https://github.com/lucapette/fakedata) cli tool. After you've installed
`fakedata` to your path, run the `make fake1gb` or `make fake5gb` commands to create
some fake data!

---

The next section reviews the setup requried for Varlog on bare metal. You can
[click here](docker_setup.md) for the docker setup.
