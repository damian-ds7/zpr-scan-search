# Przeszukiwanie skanów za pomocą OCR

**Dokumentacja wstępna projektu**

Tomasz Smoleń

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

| Warstwa                       | Technologia                                              |
| ----------------------------- | -------------------------------------------------------- |
| Interfejs CLI i logika główna | Rust (`clap`)                                            |
| Indeksowanie i przeszukiwanie | Rust                                                     |
| Ekstrakcja tekstu i OCR       | Python (`pytesseract`, `OpenCV`, `PyMuPDF`, `pdf2image`) |

## 2. Podział na podproblemy

### 2.1 Klasyfikacja i wczytanie pliku wejściowego

**Problem:** Aplikacja powinna akceptować zarówno pojedyncze pliki, jak i
katalogi zawierające wiele dokumentów. Plik PDF może zawierać tekst, obrazy lub
jedno i drugie – sposób przetwarzania zależy od tej klasyfikacji.

**Rozwiązanie:** Moduł wejściowy (Rust) zbiera listę obsługiwanych plików i
przekazuje ją do modułu Pythona, który klasyfikuje każdy plik – sprawdza, czy
PDF zawiera warstwę tekstową, czy jest skanem wymagającym OCR. Użyta biblioteka:
`PyMuPDF`.

### 2.2 Ekstrakcja tekstu z pliku PDF z warstwą tekstową

**Problem:** Pliki PDF tworzone cyfrowo zawierają wbudowaną warstwę tekstową,
którą można odczytać bezpośrednio – bez angażowania OCR.

**Rozwiązanie:** Tekst wraz z pozycjami słów na stronie zostanie wydobyty
bezpośrednio z warstwy tekstowej dokumentu. Użyta biblioteka: `PyMuPDF`.

### 2.3 Ekstrakcja tekstu z obrazów i skanów PDF (OCR)

**Problem:** Skany i zdjęcia dokumentów nie zawierają warstwy tekstowej; tekst
musi zostać rozpoznany automatycznie.

**Rozwiązanie:** Strony skanów zostaną poddane wstępnej obróbce obrazu w celu
poprawy jakości (odszumianie, poprawa kontrastu), a następnie rozpoznaniu tekstu
wraz z pozycjami słów za pomocą OCR. W przypadku skanów w formacie PDF strony
zostaną uprzednio skonwertowane do formatu graficznego. Użyte biblioteki:
`OpenCV`, `pdf2image`, `pytesseract` (silnik Tesseract).

### 2.4 Obsługa dokumentów mieszanych (PDF z osadzonymi obrazami)

**Problem:** Dokumenty mogą zawierać zarówno strony z warstwą tekstową, jak i
strony będące skanami lub zawierające osadzone obrazy z tekstem.

**Rozwiązanie:** Każda strona dokumentu jest analizowana niezależnie i kierowana
na odpowiednią ścieżkę przetwarzania – bezpośrednią ekstrakcję tekstu lub OCR.
Wyniki z obu ścieżek są scalane w ujednolicony format. Użyta biblioteka:
`PyMuPDF`.

### 2.5 Indeksowanie i cache

**Problem:** Wielokrotne uruchamianie OCR na tych samych plikach jest kosztowne
czasowo. Wyniki powinny być przechowywane lokalnie i inwalidowane przy zmianie
pliku źródłowego.

**Rozwiązanie:** Dla każdego przetworzonego pliku tworzony jest ukryty plik
cache (`.nazwa_pliku.cache`) w formacie JSON, zawierający wyekstrahowany tekst,
indeks pozycyjny (mapa słowo → pozycje wystąpień w danym pliku) oraz sumę
kontrolną pliku źródłowego. Przy kolejnym uruchomieniu program weryfikuje sumę
kontrolną i pomija ponowne przetwarzanie, jeśli plik nie uległ zmianie.

Przykładowa struktura indeksu pozycyjnego w pliku cache:

```json
{
    "Ala":  [2, [500, 750]],
    "ma":   [35, [7, 20, "..."]],
    "kota": [7, [15, 30, "..."]]
}
```

Gdzie pierwsza wartość to liczba wystąpień, a druga to lista pozycji w
dokumencie.

### 2.6 Przeszukiwanie tekstu

**Problem:** Użytkownik podaje zapytanie (jedno lub kilka słów) i oczekuje listy
dokumentów oraz lokalizacji, gdzie te słowa wystąpiły.

**Rozwiązanie:** Przy wyszukiwaniu Rust wczytuje pliki cache wszystkich
zaindeksowanych dokumentów i łączy indeksy pozycyjne w jeden indeks odwrócony w
pamięci – mapujący słowo na listę plików i pozycji wystąpień w każdym z nich.
Dla zapytań wielosłownych algorytm startuje od tokenu z najmniejszą liczbą
wystąpień, minimalizując tym samym liczbę porównań. Tokeny zapytania są
normalizowane (małe litery, opcjonalnie sprowadzanie polskich znaków
diakrytycznych do ich odpowiedników ASCII). Wynikiem jest lista trafień z nazwą
pliku, numerem strony i koordynatami.

### 2.7 Interfejs wiersza poleceń (CLI)

**Problem:** Użytkownik potrzebuje prostego sposobu na korzystanie z programu z
linii poleceń.

**Rozwiązanie:** Interfejs napisany w Rust z użyciem biblioteki `clap`. Program
udostępnia polecenia do skanowania plików lub katalogów, przeszukiwania
zaindeksowanych dokumentów oraz czyszczenia cache'u.
