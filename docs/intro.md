# Przeszukiwanie skanów za pomocą OCR

**Dokumentacja wstępna projektu**

Tomasz Smoleń\
Damian D'Souza

## 1. Opis zadania

Celem projektu jest stworzenie narzędzia terminalowego umożliwiającego
**ekstrakcję i przeszukiwanie tekstu** z różnych typów dokumentów: plików PDF
(zarówno z warstwą tekstową, jak i będących skanami), obrazów (np. JPEG, PNG)
oraz dokumentów mieszanych zawierających zarówno tekst, jak i osadzone obrazy.

Program będzie udostępniał trzy główne funkcjonalności:

1. **Wczytanie** wskazanego katalogu lub pojedynczego pliku (PDF lub obraz).
1. **Ekstrakcję tekstu** z pliku z wykorzystaniem OCR oraz zapis wyników do
   lokalnego cache'u.
1. **Przeszukiwanie** wcześniej zaindeksowanych dokumentów z podaniem
   lokalizacji trafień.

### Stos technologiczny

| Warstwa              | Technologia                            |
| -------------------- | -------------------------------------- |
| Interfejs CLI        | Python (`clip`)                        |
| Logika przetwarzania | Rust (biblioteka wywoływana z Pythona) |

## 2. Podział na podproblemy

### 2.1 Klasyfikacja i wczytanie pliku wejściowego

**Problem:** Aplikacja powinna akceptować zarówno pojedyncze pliki, jak i
katalogi zawierające wiele dokumentów. Plik PDF może zawierać tekst, obrazy lub
jedno i drugie – sposób przetwarzania zależy od tej klasyfikacji.

**Rozwiązanie:** Warstwa Python odpowiada za wczytanie plików i ich przekazanie
do biblioteki w Rust. Klasyfikacja pliku oraz wybór ścieżki przetwarzania
realizowane są w Rust.

### 2.2 Ekstrakcja tekstu z pliku PDF z warstwą tekstową

**Problem:** Pliki PDF tworzone cyfrowo zawierają wbudowaną warstwę tekstową,
którą można odczytać bezpośrednio – bez angażowania OCR.

**Rozwiązanie:** Ekstrakcja tekstu oraz pozycji słów realizowana jest w Rust
jako część biblioteki przetwarzającej dokumenty (np. z wykorzystaniem bibliotek
takich jak `lopdf`, `pdfium-render` lub `poppler`).

### 2.3 Ekstrakcja tekstu z obrazów i skanów PDF (OCR)

**Problem:** Skany i zdjęcia dokumentów nie zawierają warstwy tekstowej; tekst
musi zostać rozpoznany automatycznie.

**Rozwiązanie:** Przetwarzanie obrazów oraz OCR realizowane są w Rust. Proces
obejmuje wstępne przetwarzanie obrazu oraz rozpoznawanie tekstu (np. z
wykorzystaniem `tesseract-rs` oraz bibliotek do przetwarzania obrazu takich jak
`image` lub `opencv`).

### 2.4 Obsługa dokumentów mieszanych (PDF z osadzonymi obrazami)

**Problem:** Dokumenty mogą zawierać zarówno strony z warstwą tekstową, jak i
strony będące skanami lub zawierające osadzone obrazy z tekstem.

**Rozwiązanie:** Każda strona dokumentu jest analizowana w Rust i kierowana na
odpowiednią ścieżkę przetwarzania. Wyniki są scalane w ujednolicony format.

### 2.5 Indeksowanie i cache

**Problem:** Wielokrotne uruchamianie OCR na tych samych plikach jest kosztowne
czasowo. Wyniki powinny być przechowywane lokalnie i inwalidowane przy zmianie
pliku źródłowego.

**Rozwiązanie:** Mechanizm cache oraz indeksowania jest zaimplementowany w Rust
(jako biblioteka). Dla każdego pliku tworzony jest plik cache zawierający
wyekstrahowany tekst, indeks pozycyjny oraz sumę kontrolną pliku. Przy kolejnym
uruchomieniu weryfikowana jest zgodność sumy kontrolnej.

Przykładowa struktura indeksu pozycyjnego:

```json
{
    "Ala":  [2, [500, 750]],
    "ma":   [35, [7, 20, "..."]],
    "kota": [7, [15, 30, "..."]]
}
```

### 2.6 Przeszukiwanie tekstu

**Problem:** Użytkownik podaje zapytanie i oczekuje listy dokumentów oraz
lokalizacji wystąpień.

**Rozwiązanie:** Logika przeszukiwania znajduje się w Rust. System korzysta z
indeksów zapisanych w cache i wykonuje wyszukiwanie na podstawie zapytania. Dla
zapytań wielosłownych możliwa jest optymalizacja poprzez wybór tokenu o
najmniejszej liczbie wystąpień.

### 2.7 Interfejs CLI

**Problem:** Użytkownik potrzebuje prostego sposobu korzystania z programu.

**Rozwiązanie:** Interfejs CLI zaimplementowany w Pythonie. Odpowiada za
przyjmowanie poleceń użytkownika oraz wywoływanie funkcji biblioteki Rust
odpowiedzialnej za całe przetwarzanie (ekstrakcję, cache i przeszukiwanie).
