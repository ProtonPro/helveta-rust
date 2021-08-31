pub mod pe {
    use std::collections::HashMap;
    use std::ffi::CStr;
    use std::str;
    use winapi::um::{
        libloaderapi::GetModuleHandleA,
        winnt::{LPCSTR, PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS, PIMAGE_SECTION_HEADER},
    };

    #[derive(Debug)]
    pub struct Section {
        name: &'static str,
        start: u32,
        end: u32,
    }

    pub struct Dll {
        name: LPCSTR,
        module: *const u8,
        size: u32,
        sections: HashMap<&'static str, Section>,
    }

    impl Dll {
        pub fn new(name: LPCSTR) -> Self {
            unsafe {
                //	prepare module data
                let module = GetModuleHandleA(name) as *const u8;
                let dos_header = module as PIMAGE_DOS_HEADER;
                let nt_headers = ((module as i32) + (*(dos_header)).e_lfanew) as PIMAGE_NT_HEADERS;
                let size = (*(nt_headers)).OptionalHeader.SizeOfImage;

                //	prepare local hashmap to pass after work
                let mut sections: HashMap<&'static str, Section> = HashMap::new();

                //	image_first_section of nt_headers
                let mut section_list = ((nt_headers as usize)
					//	offset of OptionalHeader field in IMAGE_NT_HEADERS
                    + 24
                    + (*(nt_headers)).FileHeader.SizeOfOptionalHeader as usize)
                    as PIMAGE_SECTION_HEADER;

                //	run work and go to the next section
                for _n in 0..(*(nt_headers)).FileHeader.NumberOfSections {
                    //	find null terminator in array and slice at it then form string
                    let name_as_mut = (*section_list).Name.as_mut();
                    let name_nul = name_as_mut
                        .iter()
                        .position(|&c| c == b'\0')
                        .unwrap_or(name_as_mut.len());
                    let name = str::from_utf8_unchecked(&name_as_mut[0..name_nul]);

                    sections.insert(
                        name,
                        Section {
                            name,
                            start: (*section_list).PointerToRawData,
                            end: (*section_list).SizeOfRawData,
                        },
                    );

                    //	image_section_header size is 40
                    section_list = (section_list as u32 + 40) as PIMAGE_SECTION_HEADER;
                }

                Self {
                    name,
                    module,
                    size,
                    sections,
                }
            }
        }

        //	information
        pub fn get_raw_name(&self) -> LPCSTR {
            self.name
        }

        pub fn get_name(&self) -> &str {
            unsafe { CStr::from_ptr(self.get_raw_name()).to_str().unwrap() }
        }

        pub fn get_module(&self) -> *const u8 {
            self.module
        }

        pub fn get_address(&self, indice: u32) -> u32 {
            self.module as u32 + indice
        }

        pub fn get_ptr(&self, indice: u32) -> *const u8 {
            self.get_address(indice) as *const u8
        }

        pub fn get_opcode(&self, indice: u32) -> u8 {
            unsafe { *self.get_ptr(indice) }
        }

        pub fn get_size(&self) -> u32 {
            self.size
        }

        pub fn get_sections(&self) -> &HashMap<&'static str, Section> {
            &self.sections
        }

        pub fn get_section(&self, name: &'static str) -> Option<&Section> {
            self.sections.get(name)
        }

        //	utilities
        pub fn pattern_scan_impl(&self, sig: &[u8], section: &'static str, nth: i32) -> u32 {
            let sig_size = sig.len();
            let scan_start: u32;
            let scan_size: u32;

            match self.get_section(section) {
                None => {
                    scan_start = 0x1000;
                    scan_size = self.get_size() - sig_size as u32;
                }
                Some(x) => {
                    scan_start = x.start;
                    scan_size = x.end - sig_size as u32;
                }
            }

            let mut nth_find = 0;
            for i in scan_start..scan_size {
                let mut found = true;
                for j in 0..sig_size {
                    let indice = i + j as u32;
                    let opcode = self.get_opcode(indice);
                    if opcode != sig[j] && sig[j] != 0xCC {
                        found = false;
                        break;
                    }
                }

                if found {
                    nth_find += 1;

                    if nth_find != nth {
                        continue;
                    }

                    return self.get_address(i);
                }
            }

            0
        }

        pub fn pattern_scan_section(&self, sig: &[u8], name: &'static str) -> u32 {
            self.pattern_scan_impl(sig, name, 1)
        }

        pub fn pattern_scan(&self, sig: &[u8]) -> u32 {
            self.pattern_scan_section(sig, ".text")
        }
    }

    //	debug information specialization
    impl std::fmt::Debug for Dll {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Dll")
                .field("name", &self.get_name())
                .field("module", &self.get_module())
                .field("size", &self.get_size())
                .field("sections", &self.get_sections())
                .finish()
        }
    }
}
