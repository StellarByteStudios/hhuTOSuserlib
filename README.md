# hhuTOSuserlib
Eine Library, welche eine gemeinsame Syscall-Schnittstelle für mehrere hhuTOS Betriebssysteme bereitstellen soll. Entwickelt für das an der HHU beigebrachte hhuTOS

---

## Besprechung
- Syscall für Thread.exit() um bei Panics den Thread zu Enden
- Syscall für kprint mit nicht nur Zahlen
- Man braucht eine "Dummy-Main" im Kernel, da die Userlib implementiert wird
- Panic-Handler aus Apps entfernen
- Neues Erstellen von Apps
  - Environment muss jetzt geladen werden
    - Neues Mapping im Userspace
    - Händisches Kopieren der Argumente in den Speicher


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
 