mod type_urls;

pub const NEUTRON_VERSION: &str = include_str!("prost/neutron/NEUTRON_COMMIT");

pub mod neutron {
    pub mod contractmanager {
        include!("prost/neutron/neutron.contractmanager.rs");
    }

    pub mod cron {
        include!("prost/neutron/neutron.cron.rs");
    }

    pub mod feeburner {
        include!("prost/neutron/neutron.feeburner.rs");
    }

    pub mod feerefunder {
        include!("prost/neutron/neutron.feerefunder.rs");
    }

    pub mod interchainqueries {
        include!("prost/neutron/neutron.interchainqueries.rs");
    }

    pub mod interchaintxs {
        pub mod v1 {
            include!("prost/neutron/neutron.interchaintxs.v1.rs");
        }
    }

    pub mod transfer {
        include!("prost/neutron/neutron.transfer.rs");
    }
}
