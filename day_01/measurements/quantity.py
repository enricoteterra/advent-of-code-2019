class Quantity(object):
    def __init__(self, unit, amount):
        super(Quantity, self)
        self.unit = unit
        self.amount = amount

    def __add__(self, other):
        if other.unit is not self.unit:
            return self
        return Quantity(amount=self.amount + other.amount, unit=self.unit)

    def __str__(self):
        return str(self.amount) + " " + str(self.unit.value)

    def __eq__(self, other):
        return self.unit is other.unit and self.amount == other.amount
