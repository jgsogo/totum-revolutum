
# TODO: Only some MovementTypes are allowed for some AccountTypes.
# By default everything is allowed or the opposite?
# I guess all the nodes below in the hierarchy are included --> we should sanitize (maybe keep the list of nodes, as the hierarchy_tree already does)
# See notes about movement_types (and movement classes) related to account_type

class M2MMovementsAllowedPerAccount(models.Model):
    movement_type = models.ForeignKey()
    account_type = models.ForeignKey()

    all_movements_list = # the full list of movement_type pks including all children
    
    movement_class_override = # If, for this specific relation some MovementXXXX class should be instantiated instead of other