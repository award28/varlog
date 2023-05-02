# Testing

Varlog has a test suite which can be run using the `make test` command. Note that due to requirements for full integration testing, the command is executed with `sudo` so that a true test can be performed for reading log files from disk.

All feature changes submitted to Varlog must include an integration test to validate the new functionality. Varlog uses [Gherkin](https://cucumber-rs.github.io/cucumber/current/introduction.html) for integraiton tests, which can be found in the `server/tests/features` directory. We recommend using the existing Gherkin statements available where ever possible. However, in the case that you need to create a new case, please review the provided Gherkin documentation and add your case to the `inteagration_tests.rs` file.
