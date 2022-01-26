# NTP - Optimizacija organizacije rasporeda nastave (Schedule)
Predmetni projekat iz naprednih tehnika programiranja - Romana Erdelji, sw58/2017

## Opis problema
Rešavanje Nurse Scheduling Problem-a (NSP) adaptiran na problem organizacije rasporeda nastave u odnosu na zahteve fakulteta i želje 
(koji dani bi im više odgovarali u odnosu na druge) profesora i asistenata (hard and soft constraints).

## Opis rešenja
Za rešavanje problema bi se koristio genetski algoritam. <br />
Algoritam bi bio implementiran u programskom jeziku Go i u programskom jeziku Rust, kako bi se poredile performanse i težina implementacije.

## Opis rada aplikacije
U schedule aplikaciji postoje dve vrste role - admin i staff. <br />
Admin ima sledeće mogućnosti:
<ul>
<li> CRUD operacije nad  </li> 
  <ul>
    <li> fakultetima </li>
    <li> smenama </li>
    <li> predavanjima/vežbama </li>
    <li> profesorima/asistentima </li>
    <li> zahtevima fakulteta (soft constraints) </li>
    </ul>
<li> generisanje rasporeda nastave za izabrani fakultet kao i pregled celog rasporeda </li>
 </ul>
Staff ima sledeće mogućnosti:
<ul>
<li> da update-uje svoje želje (soft constraints) </li>
<li> vidi svoja predavanja </li>
<li> i vidi raspored nastave za sebe ukoliko ga je admin generisao </li>
</ul>

## Arhitektura sistema
Arhitektura sistema prestavlja dva backend-a sa deljenim client side. <br />
Go backend koristi MongoDB za skladištenje podataka dok Rust koristi Postgres. <br />
Za svaki deo je namešten docker image. <br />
Note: rust je rađen kao mikroservisna arhitektura radi diplomskog rada

### Go
Monolitna aplikacija koja predstavlja backend side, implementira CRUD operacije za sledeće entitete: fakultet, smene, predavanja/vežbe, profesore/asistente, hard i soft constraints, i genetski algoritam za generisanje rasporeda nastave.
![Go drawio](https://user-images.githubusercontent.com/45543511/151180235-9974c496-7c82-43f1-8a59-72cc69e25a0a.png)

### Rust
Mikroservisna aplikacija koja predstavlja backend side.
Sastoji se iz sledećih mikroservisa:
* api-gateway - prosleđuje http request odgovarajućem mikroservisu
* auth-service - obavlja process login-a
* faculty-service - CRUD operacije za fakultet, smene i zahteve fakulteta (hard constraints)
* lecture-service - CRUD operacije za predavanja/vežbe
* schedule-service - generisanje (genetski algoritam) i dobavljanje rasporeda nastave
* staff-service - CRUD operacije za profesore/asistente i soft constraints

Mikroservis schedule-service takođe komunicira sa faculty-service, lecture-service i staff-service radi dobavljanja potrebnih podataka za rad algoritma.

![Rust-MS drawio](https://user-images.githubusercontent.com/45543511/151181509-8cef18b7-163c-49a4-99cc-87c3f62ed097.png)

### Angular
Predstavlja client side aplikacije, služi za manipulisanje CRUD operacijama nad entitetima i prikaz rasporeda nastave u tabelarnom prikazu.

### NSP algoritam
Deljan opis algoritma, kao i poređenje performansi i težina implementacije se nalazi u fajlu "Izveštaj".

## Docker
U folderu "docker-compose files" se nalaze compose fajlovi koji će napraviti container-e za go, rust, angular i baze. 
Pre puštanja compose fajla za go i rust potrebno je namestiti sledeće txt fajlove na putanji "D:/logs":
* golang
* logs_api_gateway
* logs_auth_service
* logs_faculty_service
* logs_lecture_service
* logs_schedule_service
* logs_staff_service
