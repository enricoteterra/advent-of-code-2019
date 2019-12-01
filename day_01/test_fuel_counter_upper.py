import unittest
from fuel_counter_upper import fuel_requirements_of, compound_fuel_requirements_of, Mass, Fuel


class TestFuelCounterUpper(unittest.TestCase):

    def test_fuel_requirements_of(self):
        self.assertEqual(fuel_requirements_of(Mass(0)), Fuel(0))
        self.assertEqual(fuel_requirements_of(Mass(-1)), Fuel(0))
        self.assertEqual(fuel_requirements_of(Mass(-100)), Fuel(0))

        self.assertEqual(fuel_requirements_of(Mass(12)), Fuel(2))
        self.assertEqual(fuel_requirements_of(Mass(14)), Fuel(2))
        self.assertEqual(fuel_requirements_of(Mass(1969)), Fuel(654))
        self.assertEqual(fuel_requirements_of(Mass(100756)), Fuel(33583))

    def test_compount_fuel_requirements_of(self):
        self.assertEqual(compound_fuel_requirements_of(Mass(1969)), Fuel(966))
        self.assertEqual(compound_fuel_requirements_of(
            Mass(100756)), Fuel(50346))


if __name__ == '__main__':
    unittest.main()
