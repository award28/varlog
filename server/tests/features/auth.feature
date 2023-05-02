Feature: Auth feature

  Scenario: If we request acceptable access we're granted a token
    Given I need path access .* and server access .*
    When I request access
    Then I receive status code 200
    And I receive a token
    And I receive an expires

  Scenario: If we request parent directory access it's denied
    Given I need path access ..* and server access .*
    When I request access
    Then I receive status code 400
    And I receive an error
    And the error contains 'Parent directory access is forbidden.'

  Scenario: If we request an unrecognizable path
    Given I need path access * and server access .*
    When I request access
    Then I receive status code 400
    And I receive an error
    And the error contains 'Invalid file paths.'
