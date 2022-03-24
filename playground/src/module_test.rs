
mod plant_structures {
    pub mod roots {
        pub mod products {
            pub(in crate::module_test::plant_structures::roots) struct Cytokinin {

            }
        }

        use products::Cytokinin; // これはエラーにならない
    }

    // use roots::products::Cytokinin; これはエラーになる
    pub mod stems {

    }

    pub mod leaves {

    }
}