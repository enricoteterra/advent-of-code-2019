from decimal import Decimal
from enum import Enum
from measurements.quantity import Quantity
from measurements.mass import METRIC


def Fuel(fuel): return Quantity(amount=Decimal(fuel), unit=METRIC.KG)
