Feature: Dlc Link Stack sample feature

  Scenario: Create and attest oracle event
    Given an oracle backend client with address http://localhost:8080
    When creating a new oracle event 'myevent' with uuid abc123
    When getting an attestation 'myattest' with uuid abc123 and outcome: 12345
    Then expected status code for 'myevent' is 200
    Then expected status code for 'myattest' is 200
