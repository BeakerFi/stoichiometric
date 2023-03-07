use scrypto::blueprint;

#[blueprint]
mod router {

    use crate::pool::PoolComponent;

    pub struct Router{
        pools: HashMap<(ResourceAddress, ResourceAddress), PoolComponent>,
        position_minter: Vault,
        position_address: ResourceAddress,
        position_id: u64,
        admin_badge: ResourceAddress
    }

    impl Router {

        pub fn new() -> (ComponentAddress, Bucket) {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Router admin badge")
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            // Creates the position minter
            let position_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(Decimal::ONE);

            // Creates the NFR Position address
            let position_resource = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric Position")
                .mintable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_non_fungible_data(
                    rule!(require(position_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            let router_rules = AccessRules::new()
                .default(
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll
                );

            let mut component = Self {
                pools: HashMap::new(),
                position_minter: Vault::with_bucket(position_minter),
                position_address: position_resource,
                position_id: 0,
                admin_badge: admin_badge.resource_address()
            }
                .instantiate();

            component.add_access_check(router_rules);
            let component = component.globalize();
            (component, admin_badge)

        }
    }


}