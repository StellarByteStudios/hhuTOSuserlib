# hhuTOSuserlib
Eine Library, welche eine gemeinsame Syscall-Schnittstelle für mehrere hhuTOS Betriebssysteme bereitstellen soll. Entwickelt für das an der HHU beigebrachte hhuTOS

---

## Ideen für weitere Funktionen
### Privatisieren der Syscalls
Es wäre vielleicht cleaner, wenn man die direkten Syscalls nicht mehr von den Anwendungen aus aufrufen kann. Eine Möglichkeit wäre, man stellt für alle Syscallfunktionen Wrapper in anderen packages zu verfügung. Diese können dann auch gewisse Vorverarbeitung und Nachbereitung machen, wie z.B. das umwandeln in einen String, wenn man nach dem Prozessnamen fragt.

### Besseres Format für Musik
Aktuell werden nur Musik-Stücke, welche der Kernel kennt über eine ID abgespielt. Sinnvoller wäre aber, dass die Anwendung selbst Musik erstellen könnte. Dazu bräuchte man ein Musikformat, womit man einen Buffer von Noten übertragen könnte.

### Shellfunktionen in den Usermode schieben
Aktuell läuft die Shell selbst im Kernel. Besser wäre das im Usermode als Anwendung bräuchte dafür aber zuverlässige möglichkeit, die Tastatur einzulesen
--> neuen Syscall: getKey
