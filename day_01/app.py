from measurements.mass import Mass
from measurements.fuel import Fuel
from fuel_counter import compound_fuel_requirements_of
from math import floor


def fuel_equation(mass): return Fuel(floor(mass.amount / 3) - 2)


file = open('data/module-mass.txt', 'r')
total_fuel_required = Fuel(0)
for module in file.readlines():
    total_fuel_required = total_fuel_required + \
        compound_fuel_requirements_of(Mass(module), fuel_equation)
file.close()
print(total_fuel_required)
