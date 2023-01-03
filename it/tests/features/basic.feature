Feature: Dlc Link Stack sample feature

  Scenario: Create and accept an offer
    Given a wallet backend client with address http://localhost:8085
    Given an oracle backend client with address http://localhost:8080
    When creating a new oracle event with uuid 0xfdc34ee06024ad476e18fff8a80faa5d14dc427cfff3131642c77eaff6bc1d9a
    When creating an offer request with uuid 0xfdc34ee06024ad476e18fff8a80faa5d14dc427cfff3131642c77eaff6bc1d9a, accept_collateral: 100000000 and offer_collateral: 10000
    When getting an attestation with uuid 0xfdc34ee06024ad476e18fff8a80faa5d14dc427cfff3131642c77eaff6bc1d9a and outcome: 12345
    When accept message: 0xfdc34ee06024ad476e18fff8a80faa5d14dc427cfff3131642c77eaff6bc1d9a
    Then expected status code is 200
