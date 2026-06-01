# Ocr scan search - dokumentacja końcowa

Tomasz Smoleń\
Damian D'Souza

# Uruchomienie

Uruchomienie aplikacji wymaga uruchomienie za pomocą `uv` skryptu `search.py`. `Maturin` sam zajmie się zbudowaniem części rustowej aplikacji.

# Architektura

Diagramy architektury znajdują się w plikach `architecture_uml.html/pdf`, do ich wygenerowania użyliśmy wtyczki mermaid do `.md`

# Osiągnięta funkcjonalność aplikacji

- Ekstrakcja tekstu z plików `pdf` oraz zdjęć za pomocą silnika ocr

- Tworzenie i odczytywanie cache pliku, o ile nie był zmodyfikowany. W cache znajdują się:

- - wyekstraktowany tekst
  
  - odwrócony indeks słów w tekście
  
  - zanurzenia wygenerowane za pomocą modelu enkoder, używane później do wyszukwiania semantycznego

- Zapis cache odbywa się asynchronicznie za pomocą `Writera`

- Przeszukiwanie indeksowe - korzysta z odwróconego indeksu by znacznie przyśpieszyć wyszukiwanie, zaczyna od najrzadszego wyszukiwanego słowa, aby zminimalizować ilość porównań.

- Przeszukiwanie sematyczne - porównuje zanurzenia `query` z kolejnymi liniami w tekście i zwraca najbardziej podobne linie.

- Przeszukiwanie indeksowe i semantyczne domyślnie korzystają z systemowego [pagera](https://en.wikipedia.org/wiki/Terminal_pager).

- Wyszukiwanie interaktywne - korzysta z prostej aplikacji `Textual`, która imituje pager, ale pozwala na interaktwyne zmienianie wyszukiwanej frazy, lecz nie posiada wielu funkcjonalności/skrótów klawiszowych systemowych pagerów.

- Config w którym można zdefiniować rzecz takie jak: Model do zanurzeń, opcje przeszukiwania folderów, języki do ocr.

# Nie osiągnięta funkcjonalność aplikacji

- "Przeszukiwanie wcześniej zaindeksowanych dokumentów z podaniem
  lokalizacji trafień."  z dokumentacji wstępnej. W nasyzm projekcie postanowiliśmy się skupic na surowym wyekstraktowanym tekście i w okół niego skupić całą funkcjonalność. Dodatkowo zaimplementowaliśmy wyszukiwanie semantyczne, którego oryginalnie nie było w planach. Cieżkie też to byłoby do zaimplementowania w postaci aplikacji cli, którą zakładaliśmy odpoczątku.

# Powody do dumy

- Cały pipeline po przeprowadzeniu początkowego cachowania jest rzeczywiście bardzo szybki. Wyszukiwanie indeksowe jest praktycznie natychmiastowe, a semantyczne też nie zostaje daleko w tyle.

- Aplikacja jest napisana w sposób który bardzo ułatwia dodawanie nowych funkcjonalności, np. stworzenie systemu trzymającego cache na jakimś serwerze byłoby bardzo proste dzięki traitowi `CacheBackend`, a dodanie nowego rodzaju wyszukiwania (np. naiwnego, liniowego) też wymagałoby tylko implementację traita `Search`

- Duża część aplikacji jest napisana w sposób `thread-safe`, zapisywanie cachu do pliku jest tego przykładem. 

- Podczas tworzenia projektu korzystaliśmy z narzędzi takich jak:

- - ci/cd
  
  - feature branche
  
  - code review oraz mergowanie tylko bo "approvie" drugiej osoby
  
  - conventional commits

- Wszystkie bardzo nam ułatwiły życie, ale są też oczywiście bardzo dobrymi praktykami programistycznymi
