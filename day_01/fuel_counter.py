from decimal import Decimal
from math import floor
from measurements.quantity import Quantity
from measurements.fuel import Fuel


def fuel_requirements_of(mass, fuel_equation):
    """find the fuel required for a module of the given mass"""
    if mass.amount < 1:
        return Fuel(0)

    return fuel_equation(mass)


def compound_fuel_requirements_of(mass, fuel_equation):
    """fuel adds to the total mass so also requires fuel"""
    fuel = fuel_requirements_of(mass, fuel_equation)
    if fuel.amount < 1:
        return Fuel(0)

    return fuel + compound_fuel_requirements_of(fuel, fuel_equation)
