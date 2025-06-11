# hhuTOSuserlib
Eine Library, welche eine gemeinsame Syscall-Schnittstelle für mehrere hhuTOS Betriebssysteme bereitstellen soll. Entwickelt für das an der HHU beigebrachte hhuTOS

---

## Besprechung
- Syscall für ListApps -> Direkt Print
- Syscall für ListRunning -> Direkt Print
- Syscall für Musik
  - Noten Struct: Liste wird im Syscall übergeben
- Apps entwickeln



## Ideen für weitere Funktionen
### Privatisieren der Syscalls
Es wäre vielleicht cleaner, wenn man die direkten Syscalls nicht mehr von den Anwendungen aus aufrufen kann. Eine Möglichkeit wäre, man stellt für alle Syscallfunktionen Wrapper in anderen packages zu verfügung. Diese können dann auch gewisse Vorverarbeitung und Nachbereitung machen, wie z.B. das umwandeln in einen String, wenn man nach dem Prozessnamen fragt.

### Besseres Format für Musik
Aktuell werden nur Musik-Stücke, welche der Kernel kennt über eine ID abgespielt. Sinnvoller wäre aber, dass die Anwendung selbst Musik erstellen könnte. Dazu bräuchte man ein Musikformat, womit man einen Buffer von Noten übertragen könnte.

### Shellfunktionen in den Usermode schieben
Aktuell läuft die Shell selbst im Kernel. Besser wäre das im Usermode als Anwendung bräuchte dafür aber zuverlässige möglichkeit, die Tastatur einzulesen
--> neuen Syscall: getKey



## Dokumentation

### App-Environment
- Einige Konstanten sind vom Kernel in die Lib gewandert
  - const USER_SPACE_START
  - const USER_SPACE_CODE_START
  - const USER_SPACE_ENV_START
  - const USER_SPACE_ARG_START
- Der Panic-Handler ist jetzt in der Userlib
  - Benötigt noch zwei Syscalls:
    - kill-Thread/Process
    - kprint Syscall
- Heap wird noch nicht initialisiert in Runtime (Später vielleicht) 
- VMA hat jetzt auch Environment Feld
- Userlib gibt die Methoden `args()` und `args_as_vec()` um die Argumente als Iterator oder Vektor zu bekommen
- Allocator wird jetzt auch in Runtime initialisiert



### Kernel Code vorlagen
#### Laden der Environment
Neuer Code in `threads.rs`
```rust
pub fn new_app_thread(app: AppRegion, pid: usize, args: &Vec<String>) -> Box<Thread> {
    // Entry-Thread konvertieren
    let thread_entry = unsafe { transmute::<usize, extern "C" fn()>(USER_CODE_VM_START) };
    
    let app_thread =
        Self::internal_new(thread_entry, false, pid, app.file_name.clone(), Vec::new());
    
    
    
    // ============ NEU! Environment ============ //
    // Gesammten Speicherplatz für die Argumente berechnen
    let args_size = args.iter().map(|arg| arg.len()).sum::<usize>();
    
    // Startadresse
    let env_virt_start = USER_SPACE_ENV_START;
    let env_size = args_size;
    
    // Mappen der Environment im App-Space
    // Startadresse im Virtuellen Adressraum
    kprintln!("--------------- Lege Mapping für Environment an");
    let phys_addr_of_env = pg_mmap_user_environment(pid, env_virt_start, env_size);
    
    
    
    // Aufbauen von argc und argv im Userspace
    // Im ersten Eintrag steht die Anzahl der Argumente
    let argc_phys: *mut usize = phys_addr_of_env.as_mut_ptr::<usize>();
    // Pointer einer pro Element in den Args
    let argv_phys = (phys_addr_of_env.raw() + size_of::<usize>() as u64) as *mut *mut u8;
    
    // Alle Argumente zum pointer kopieren
    unsafe {
        // Anzahl der Argumente in den Speicher schreiben
        argc_phys.write(args.len() + 1);
    
    
        // Physische Startadresse der Environment-Variablen
        // (args.len() + 1) weil davor ja nur die Pointer sind und dannach die Echten Inhalte kommen
        let args_begin_phys = argv_phys.offset((args.len() + 1) as isize).cast::<u8>();
        // Virtuelle Startadresse der Environment-Variablen
        let args_begin_virt = env_virt_start
            + size_of::<usize>() // Feld mit Anzahl Einträge
            + ((args.len() + 1) * size_of::<usize>()); // Platz für die Pointer
    
        // Programmname als erstes Argument speichern
        let name = app.file_name.clone();
        args_begin_phys.copy_from(name.as_bytes().as_ptr(), name.len());
        args_begin_phys.add(name.len()).write(0); // null-terminieren für den String
    
        // Pointer auf Anfang des Namens im Eingabearray speichern
        argv_phys.write(args_begin_virt as *mut u8);
    
        // Wo gehen die nächsten Argumente hin?
        let mut offset = name.len() + 1;
    
        // Restlichen Argumente kopieren
        for (i, arg) in args.iter().enumerate() {
            // An welche physische Adresse wird das Argument geschrieben
            let target_address = args_begin_phys.add(offset);
    
            // Den String roh in den Speicher schreiben
            target_address.copy_from(arg.as_bytes().as_ptr(), arg.len());
            target_address.add(arg.len()).write(0); // null-terminieren für den String
    
            // Pointer auf neue das Argument in unser Array schreiben
            argv_phys.add(i + 1)
                .write((args_begin_virt + offset) as *mut u8);
    
            // Offset neu berechnen
            offset += arg.len() + 1;
        }
    }
    // ============  ============ //
  
  
    // App-Image mappen
    pages::pg_mmap_user_app(pid, app_thread.pml4_addr, app);
    
    return app_thread;
  }
```

Neuer Code in `pages.rs`
```rust
pub fn pg_mmap_user_environment(pid: usize, start_address: usize, len: usize) -> PhysAddr {
    // PageTable holen
    let pml4_addr = process_handler::get_pml4_address_by_pid(pid);

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // VMA berechnen und anlegen
    let vma_end = start_address + ((len / PAGE_SIZE) + 1) * PAGE_SIZE;
    let new_vma = vma::VMA::new(start_address, vma_end, vma::VmaType::Environment);
    // Past diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);
    kprintln!("VMA für Environment angelegt");
    if !success {
        return PhysAddr::new(0);
    }



    // Wie viele Pages brauche ich für meine Argumente
    let env_page_count = (len / PAGE_SIZE) + 1;

    // mappen der Environment Pages
    pml4_thread_table.mmap_general(start_address, env_page_count, false, false, false, 0);

    // Holen der physischen Startadresse
    let raw_phys_address =  get_physical_address(pml4_addr, start_address);

    return raw_phys_address;
}

fn get_physical_address(pml4_addr: PhysAddr, virtual_address: usize) -> PhysAddr {
    // Table Adresse zum Pointer
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Durch alle Tabellen durchsteppen
    let page_table_4_entry: PageTableEntry =
        pml4_table.entries[get_index_in_table(virtual_address, 3)];

    let page_table_3: &mut PageTable =
        unsafe { &mut *(page_table_4_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_3_entry: PageTableEntry =
        page_table_3.entries[get_index_in_table(virtual_address, 2)];

    let page_table_2: &mut PageTable =
        unsafe { &mut *(page_table_3_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_2_entry: PageTableEntry =
        page_table_2.entries[get_index_in_table(virtual_address, 1)];

    let page_table_1: &mut PageTable =
        unsafe { &mut *(page_table_2_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_1_entry: PageTableEntry =
        page_table_1.entries[get_index_in_table(virtual_address, 0)];

    // Index auf die Physische Adresse
    let right_index = get_index_in_table(virtual_address, 0);

    // Physische Adresse holen
    return page_table_1.entries[right_index].get_addr();
}
```
