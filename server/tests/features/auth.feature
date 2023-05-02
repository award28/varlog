Feature: Auth feature

  Scenario: If we request parent directory access it's denied
    Given I need path access .* and server access .*
    When I request access
    Then I receive status code 200
    And I receive a token
