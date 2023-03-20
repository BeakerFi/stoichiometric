use scrypto::blueprint;

#[blueprint]
mod dao {

    use stoichiometric_dex::router::RouterComponent;
    use stoichiometric_stablecoin::issuer::IssuerComponent;

    pub struct Dao {
        dex_router: ComponentAddress,
        stablecoin_issuer: ComponentAddress,
        stablecoin_address: ResourceAddress,
        stablecoin_minter: Vault,
        protocol_admin_badge: Vault,
    }

    impl Dao {

        pub fn new() -> ComponentAddress {

            // Creates the protocol admin badge which will control everything
            let protocol_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Stoichiometric protocol admin badge")
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            // Creates the stablecoin minter
            let mut stablecoin_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .recallable(rule!(require(protocol_admin_badge.resource_address())), AccessRule::DenyAll)
                .mint_initial_supply(2);

            // Creates the stablecoin resource
            let stablecoin_address = ResourceBuilder::new_fungible()
                .divisibility(18)
                .mintable(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .burnable(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .updateable_metadata(rule!(require(stablecoin_minter.resource_address())), AccessRule::DenyAll)
                .metadata("name", "Stoichiometric USD")
                .metadata("symbol", "SUSD")
                .metadata("icon", "https://cdn-icons-png.flaticon.com/512/3215/3215346.png")
                .create_with_no_initial_supply();

            let dex_router = RouterComponent::new(protocol_admin_badge.resource_address(), stablecoin_address);
            let stablecoin_issuer = IssuerComponent::new(protocol_admin_badge.resource_address(), stablecoin_minter.take(1),stablecoin_address);

            let dao_rules = AccessRules::new()
                .default(
                    rule!(allow_all),
                    AccessRule::DenyAll,
                );

            let mut component = Self{
                dex_router,
                stablecoin_issuer,
                stablecoin_address,
                stablecoin_minter : Vault::with_bucket(stablecoin_minter),
                protocol_admin_badge: Vault::with_bucket(protocol_admin_badge)
            }
                .instantiate();

            component.add_access_check(dao_rules);

            component.globalize()
        }


    }

}