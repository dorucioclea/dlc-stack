Feature: Dlc Link Stack sample feature

  Scenario: Create and accept an offer
    Given a wallet backend client with address http://localhost:8085
    Given an oracle backend client with address http://localhost:8080
    When creating a new oracle event with uuid 123
    When creating an offer request with uuid 123, accept_collateral: 10000 and offer_collateral: 2000
    When getting an attestation with uuid 123 and outcome: 12345
    When accept message: 123
    Then expected status code is 200