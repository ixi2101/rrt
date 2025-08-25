Feature: Tuple feature

    Scenario: A tuple with w=1.0 is a point
        Given a = tuple(4.3, -4.2, 3.1, 1.0)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1
        And a.w = 1.0
        And a is a point
        And a is not a vector

    Scenario: A tuple with w=0 is a vector
    Given a = tuple(4.3, -4.2, 3.1, 0.0)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1
        And a.w = 0.0
        And a is not a point
        And a is a vector

    Scenario: point() creates tuples with w=1
    Given a = point(4, -4, 3)
        Then a.x = 4
        And a.y = -4
        And a.z = 3
        And a is a point
        And a is not a vector

    Scenario: vector() creates tuples with w=0
    Given a = vector(4, -4, 3)
        Then a.x = 4
        And a.y = -4
        And a.z = 3
        And a is not a point
        And a is a vector

    Scenario: tuple equivalency
    Given a = vector(4, -4, 3)
    And b = vector(4, -4, 3)
        Then equal(a, b)

    Scenario: tuple inequivalency
    Given a = vector(4, -4, 3)
    And b = point(4, -4, 3)
        Then not equal(a, b)

    Scenario: Adding two tuples
    Given lhs = tuple(3, -2, 5, 1)
    And rhs = tuple(-2, 3, 1, 0)
    And expected = tuple(1, 1, 6, 1)
        Then expected = lhs + rhs

    Scenario: Adding two vectors
    Given lhs = vector(3, -2, 5)
    And rhs = vector(-2, 3, 1)
    And expected = vector(1, 1, 6)
        Then expected = lhs + rhs

    Scenario: Subtracting two points
    Given p1 = point(3, 2, 1)
    And p2 = point(5, 6, 7)
    And expected = vector(-2, -4, -6)
        Then expected = p1 - p2

    Scenario: Subtracting a vector from a point
    Given p = point(3, 2, 1)
    And v = vector(5, 6, 7)
    And expected = point(-2, -4, -6)
        Then expected = p - v

    Scenario: Subtracting two vectors
    Given v1 = vector(3, 2, 1)
    And v2 = vector(5, 6, 7)
    And expected = vector(-2, -4, -6)
        Then expected = v1 - v2 

    Scenario: Subtracting a vector from the zero vector
    Given zero = vector(0, 0, 0)
    And v = vector(1, -2, 3)
    And expected = vector(-1, 2, -3)
        Then expected = zero - v

    Scenario: Negating a tuple
    Given a = tuple(1, -2, 3, -4)
    And neg_a = tuple(-1, 2, -3, 4)
        Then a negated is neg_a

    Scenario: Multiplying a tuple by a scalar
    Given a = tuple(1, -2, 3, -4)
    And b = tuple(3.5, -7, 10.5, -14)
    Then a * 3.5f = b

    Scenario: Multiplying a tuple by a scalar but real
    Given a = tuple(1, -2, 3, -4)
    And b = tuple(3, -6, 9, -12)
    Then a * 3 = b

    Scenario: Multiplying a tuple by a fraction
    Given a = tuple(1, -2, 3, -4)
    Given b = tuple(0.5, -1, 1.5, -2)
    Then a * 0.5f = b

    Scenario: Dividing a tuple by a scalar
    Given a = tuple(1, -2, 3, -4)
    And b = tuple(0.5, -1, 1.5, -2)
    Then a / 2 = b

    Scenario: Dividing a tuple by a scalar but fraction
    Given a = tuple(1, -2, 3, -4)
    And b = tuple(2, -4, 6, -8)
    Then a / 0.5f = b

    Scenario: Computing the magnitude of vector(1, 0, 0)
    Given v = vector(1, 0, 0)
    Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(0, 1, 0)
    Given v = vector(0, 1, 0)
    Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(0, 0, 1)
    Given v = vector(0, 0, 1)
    Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(1, 2, 3)
    Given v = vector(1, 2, 3)
    Then magnitude(v) = √14

    Scenario: Computing the magnitude of vector(-1, -2, -3)
    Given v = vector(-1, -2, -3)
    Then magnitude(v) = √14

        