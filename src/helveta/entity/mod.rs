pub mod entity {
    use std::collections::HashMap;
    use std::mem::transmute;

    pub type Offsets = HashMap<&'static str, HashMap<&'static str, u32>>;
    lazy_static! {
        pub static ref OFFSETS: Offsets = {
            //	ignore this temporary testing hack
            //	will remove when netvar dumper is implemented
            let mut m: HashMap<&'static str, HashMap<&'static str, u32>> = HashMap::new();
            let mut h: HashMap<&'static str, u32> = HashMap::new();
            h.insert(&"m_iHealth", 0x100);
            m.insert(&"CBasePlayer", h);

            m
        };
    }

    macro_rules! get_offset {
        ($a:expr, $b:expr) => {
            OFFSETS
                .get(&$a)
                .expect("Offsets::get failed")
                .get(&$b)
                .expect("Offsets node get failed")
        };
    }

    pub struct Player {}

    impl Player {
        pub fn get(&self, ptrdiff: &u32) -> u32 {
            unsafe { transmute::<&Player, u32>(self) + ptrdiff }
        }

        pub fn health(&self) -> &i32 {
            unsafe { &*(self.get(get_offset!("CBasePlayer", "m_iHealth")) as *const i32) }
        }
    }
}
