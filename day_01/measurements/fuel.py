from decimal import Decimal
from enum import Enum
from measurements.quantity import Quantity


class VOLUME(Enum):
    LITER = "LITER"


def Fuel(fuel): return Quantity(amount=Decimal(fuel), unit=VOLUME.LITER)
