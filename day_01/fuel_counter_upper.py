from decimal import Decimal
from enum import Enum
from math import floor
from quantity import Quantity


class METRIC(Enum):
    KG = "KG"


class VOLUME(Enum):
    LITER = "LITER"


def Mass(mass): return Quantity(amount=Decimal(mass), unit=METRIC.KG)


def Fuel(fuel): return Quantity(amount=Decimal(fuel), unit=VOLUME.LITER)


def fuel_requirements_of(mass):
    """find the fuel required for a module of the given mass"""
    if mass.amount < 1:
        return Fuel(0)

    return Fuel(floor(mass.amount / 3) - 2)


def compound_fuel_requirements_of(mass):
    """fuel adds to the total mass so also requires fuel"""
    fuel = fuel_requirements_of(mass)
    if fuel.amount < 1:
        return Fuel(0)

    return fuel + compound_fuel_requirements_of(fuel)
