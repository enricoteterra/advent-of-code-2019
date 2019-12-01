from measurements.mass import Mass
from measurements.fuel import Fuel
from fuel_counter import compound_fuel_requirements_of

file = open('data/module-mass.txt', 'r')

total_fuel_required = Fuel(0)
for module in file.readlines():
    total_fuel_required = total_fuel_required + \
        compound_fuel_requirements_of(Mass(module))

print(total_fuel_required)
file.close()
