Feature: Dlc Link Stack sample feature

  Scenario: Create and accept an offer
    Given a wallet backend client with address http://localhost:8085
    Given an oracle backend client with address http://localhost:8080
    When creating a new oracle event 'myevent' with uuid abc123
    When creating an offer request 'myoffer' with uuid abc123, accept_collateral: 100000000 and offer_collateral: 10000
    When getting an attestation 'myattest' with uuid abc123 and outcome: 12345
    When accept message: abc123 as 'myaccept'
    Then expected status code for 'myevent' is 200
    Then expected status code for 'myattest' is 200
    Then expected status code for 'myoffer' is 200
    Then expected status code for 'myaccept' is 200
