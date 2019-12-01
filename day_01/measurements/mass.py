from decimal import Decimal
from enum import Enum
from measurements.quantity import Quantity


class METRIC(Enum):
    KG = "KG"


def Mass(mass): return Quantity(amount=Decimal(mass), unit=METRIC.KG)
