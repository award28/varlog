Feature: Logs feature

  Background:
    Given there is a log file named 'some.log' with content 'hello world'

  Scenario: If we have access to some.log, we are given the logfile
    Given I have path access .* and server access .*
    When I request logs
    Then I receive status code 200
    And the logs list contains '/some.log'

  Scenario: If we don't have access to some.log, we are not given the logfile
    Given I have path access other.log and server access .*
    When I request logs
    Then I receive status code 200
    And the logs list does not contain '/some.log'

  Scenario: I have access to some.log, and can access it's contents
    Given I have path access .* and server access .*
    When I request the contents of '/some.log'
    Then I receive status code 200
    And the logs list contains 'hello world'

  Scenario: I do not have access to some.log and can't access it's contents
    Given I have path access other.log and server access .*
    When I request the contents of '/some.log'
    Then I receive status code 403
    And I receive an error
    And the error contains 'You do not have access to file'

  Scenario: I do not have access to parent directories
    Given I have path access other.log and server access .*
    When I request the contents of '/..some.log'
    Then I receive status code 403
    And I receive an error
    And the error contains 'The `..` operator is forbidden.'

  Scenario: I request a file that does not exist
    Given I have path access .* and server access .*
    When I request the contents of '/dne.log'
    Then I receive status code 400
    And I receive an error
    And the error contains 'does not exist.'
