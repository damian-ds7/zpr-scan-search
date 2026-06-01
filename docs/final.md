# Ocr scan search - dokumentacja końcowa

Tomasz Smoleń\
Damian D'Souza

# Uruchomienie

### Środowisko Docker

Wszystkie zależności systemowe (Tesseract OCR, nagłówki Python, kompilator C++)
są skonfigurowane w pliku `Dockerfile`. Aby zbudować obraz i uruchomić kontener
w trybie interaktywnym:

```bash
docker compose run --build app
```

### Uruchomienie lokalne (uv)

W przypadku pracy lokalnej należy użyć skryptu `python/search.py` przez `uv`.
`Maturin` automatycznie zbuduje część Rustową aplikacji przy zmianach w kodzie.

### Przykładowe komendy (wewnątrz kontenera lub lokalnie):

- **Podstawowe wyszukiwanie frazy:**

  ```bash
  uv run python/search.py resources -s "quick brown fox"
  ```

- **Wyszukiwanie semantyczne:**

  ```bash
  uv run python/search.py resources -sm "animal jumping over a dog"
  ```

- **Tryb interaktywny:**

  ```bash
  uv run python/search.py resources -i
  ```

- **Wyszukiwanie z określeniem języka OCR i kontekstem (2 słowa przed i po):**

  ```bash
  uv run python/search.py resources -s "sample" -l eng -c 2
  ```

- **Wymuszenie ponownego przetworzenia pliku (odświeżenie cache):**

  ```bash
  uv run python/search.py resources -r -s "fox"
  ```

# Generowanie dokumentacji

Aby wygenerować i otworzyć dokumentację biblioteki oraz modułów wewnętrznych:

```bash
cargo doc --no-deps --document-private-items --open
```

# Konfiguracja

Aplikacja pozwala na dostosowanie działania poprzez plik konfiguracyjny w
formacie TOML.

### Lokalizacja i priorytety

Program poszukuje konfiguracji w następującej kolejności, korzystając ze
standardowych lokalizacji systemowych dla danego środowiska:

1. **Flaga `--config <ścieżka>`**: Pozwala na wskazanie dowolnego pliku
   konfiguracyjnego podczas uruchamiania.
1. **Lokalizacja systemowa**: Jeśli flaga nie zostanie użyta, aplikacja szuka
   pliku w domyślnych folderach konfiguracyjnych użytkownika (np.
   `~/.config/scan-search/config.toml` w systemie Linux).
1. **Wartości domyślne**: W przypadku braku powyższych, używane są wbudowane w
   program wartości domyślne.

Można wygenerować domyślny plik konfiguracji używając flagi `--default-config`.

### Przykładowy plik konfiguracyjny

```toml
[fs_scan]
follow_links = true
include_hidden = false

[search]
sem_search = false
context_before = 5
context_after = 5

[ocr]
languages = ["eng", "pol"]

[sem_search]
model = "AllMiniLML6V2"
queue_size = 10
```

### Opis sekcji:

- **`[fs_scan]`**: Kontroluje sposób skanowania katalogów (podążanie za linkami
  symbolicznymi, uwzględnianie ukrytych plików).
- **`[search]`**: Definiuje ilość linii kontekstu wyświetlanych wokół trafienia
  oraz czy zanurzenia mają być generowane automatycznie przy pierwszym
  skanowaniu pliku (pole `sem_search`). Domyślnie zanurzenia są tworzone tylko w
  momencie wywołania wyszukiwania semantycznego.
- **`[ocr]`**: Lista języków używanych przez silnik Tesseract do rozpoznawania
  tekstu.
- **`[sem_search]`**: Konfiguracja wyszukiwania semantycznego – wybór modelu ML
  oraz rozmiar kolejki przetwarzania.

# Architektura

Diagramy architektury znajdują się w plikach `architecture_uml.html/pdf`, do ich
wygenerowania użyliśmy wtyczki mermaid do `.md`

# Osiągnięta funkcjonalność aplikacji

- Ekstrakcja tekstu z plików `pdf` oraz zdjęć za pomocą silnika ocr

- Tworzenie i odczytywanie cache pliku, o ile nie był zmodyfikowany. W cache
  znajdują się:

- - wyekstraktowany tekst

  - odwrócony indeks słów w tekście

  - zanurzenia wygenerowane za pomocą modelu enkoder, używane później do
    wyszukwiania semantycznego

- Zapis cache odbywa się asynchronicznie za pomocą `Writera`

- Przeszukiwanie indeksowe - korzysta z odwróconego indeksu by znacznie
  przyśpieszyć wyszukiwanie, zaczyna od najrzadszego wyszukiwanego słowa, aby
  zminimalizować ilość porównań.

- Przeszukiwanie sematyczne - porównuje zanurzenia `query` z kolejnymi liniami w
  tekście i zwraca najbardziej podobne linie.

- Przeszukiwanie indeksowe i semantyczne domyślnie korzystają z systemowego
  [pagera](https://en.wikipedia.org/wiki/Terminal_pager).

- Wyszukiwanie interaktywne - korzysta z prostej aplikacji `Textual`, która
  imituje pager, ale pozwala na interaktwyne zmienianie wyszukiwanej frazy, lecz
  nie posiada wielu funkcjonalności/skrótów klawiszowych systemowych pagerów.

- Config w którym można zdefiniować rzecz takie jak: Model do zanurzeń, opcje
  przeszukiwania folderów, języki do ocr.

# Nie osiągnięta funkcjonalność aplikacji

- "Przeszukiwanie wcześniej zaindeksowanych dokumentów z podaniem lokalizacji
  trafień." z dokumentacji wstępnej. W nasyzm projekcie postanowiliśmy się
  skupic na surowym wyekstraktowanym tekście i w okół niego skupić całą
  funkcjonalność. Dodatkowo zaimplementowaliśmy wyszukiwanie semantyczne,
  którego oryginalnie nie było w planach. Cieżkie też to byłoby do
  zaimplementowania w postaci aplikacji cli, którą zakładaliśmy odpoczątku.

# Powody do dumy

- Cały pipeline po przeprowadzeniu początkowego cachowania jest rzeczywiście
  bardzo szybki. Wyszukiwanie indeksowe jest praktycznie natychmiastowe, a
  semantyczne też nie zostaje daleko w tyle.

- Aplikacja jest napisana w sposób który bardzo ułatwia dodawanie nowych
  funkcjonalności, np. stworzenie systemu trzymającego cache na jakimś serwerze
  byłoby bardzo proste dzięki traitowi `CacheBackend`, a dodanie nowego rodzaju
  wyszukiwania (np. naiwnego, liniowego) też wymagałoby tylko implementację
  traita `Search`

- Duża część aplikacji jest napisana w sposób `thread-safe`, zapisywanie cachu
  do pliku jest tego przykładem.

- Podczas tworzenia projektu korzystaliśmy z narzędzi takich jak:

- - ci/cd

  - feature branche

  - code review oraz mergowanie tylko bo "approvie" drugiej osoby

  - conventional commits

- Wszystkie bardzo nam ułatwiły życie, ale są też oczywiście bardzo dobrymi
  praktykami programistycznymi
