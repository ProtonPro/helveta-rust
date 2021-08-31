mod entity;
mod pe;

pub mod helveta {
    //	local utility
    use crate::helveta::entity::entity::*;
    // use crate::helveta::entity::offsets::*;
    use crate::helveta::pe::pe::*;

    //	winapi crate
    use winapi::um::{consoleapi::AllocConsole, wincon::SetConsoleTitleA};

    macro_rules! c_str {
        ($a:expr) => {
            (concat!($a, '\0')).as_ptr() as _
        };
    }

    /// software contents, doesn't require debug information print specialization
    #[derive(Debug)]
    pub struct Context {
        client: Dll,
        // client_class_head: *const ClientClass,
        local: *const Player,
    }

    impl Context {
        pub unsafe fn new() -> Self {
            AllocConsole();
            SetConsoleTitleA(c_str!("helveta"));

            //	Setup modules
            let client = Dll::new(c_str!("client.dll"));

            //	Setup pointers
            let local = **((client.pattern_scan(
                b"\x83\x3D\xCC\xCC\xCC\xCC\xCC\x75\x68\x8B\x0D\xCC\xCC\xCC\xCC\x8B\x01",
            ) + 2) as *const *const *const Player);

            //	@todo: implement netvar dumper
            // let client_class_head: *const ClientClass;
            // unsafe {
            //     client_class_head = **((client
            //         .pattern_scan(b"\xA1\xCC\xCC\xCC\xCC\x8B\x4E\x0C\x85\xC0\x74\x18\x0F\x1F")
            //         + 1)
            //         as *const *const *const ClientClass);
            // }

            Self {
                client,
                // client_class_head,
                local,
            }
        }

        ///	dereference to player
        pub fn get_local_player(&self) -> &Player {
            unsafe { &*self.local }
        }

        ///	events and information dispatcher
        pub fn run(&self) {
            //	display all fields of context
            println!("Context:\n{:?}", self);

            //	no safety checks right now
            println!("{:?}", self.get_local_player().health());
        }
    }
}
