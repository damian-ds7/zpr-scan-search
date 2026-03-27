# Przeszukowanie skanów za pomocą OCR

## Temat i wstęp

Tematem projektu jest stworzenie aplikacji pozwalającej odczytac i przeszukać
zawartość skanów/pdf-ów. Aplikacja będzie miałą trzy główne funkcjonalności:

1. Wczytanie katalogu bądź pliku pdf bądź zdjęcia
1. Ekstrakcja tekstu z pliku oraz cachowanie go
1. Przeszukiwanie pliku

Program będzie składał się z nastepujących komponentów.

1. Bardzo prosty i minimalistyczny frontend terminalowy napisany w Rust
1. Moduł odpowiadający za rozpoznanie języka, oraz ekstrakcję za pomocą tekstu
   za pomocą OCR w Pythonie
1. Moduł odpowiadający za cachowanie tekstu, poprzez zapis.
1. Moduł odpowiadający za przeszukiwanie tekstu, za pomocą mapy hashującej

## Główne podproblemy i ich wykonanie

### OCR

Moduł OCR zostanie zaimplementowany w języku Python i będzie oparty na silniku
Tesseract (wykorzystując bibliotekę `pytesseract`). Jego zadaniem jest
przekształcenie materiałów wejściowych (skanów, zdjęć, PDF-ów) w tekst możliwy
do późniejszego indeksowania i przeszukiwania.

- **Przetwarzanie obrazów**: Surowe skany lub zdjęcia będą poddawane wstępnej
  obróbce przy użyciu biblioteki `OpenCV`. Celem jest optymalizacja jakości
  obrazu (np. poprzez odszumianie czy poprawę kontrastu), co bezpośrednio
  przekłada się na wyższą skuteczność rozpoznawania znaków.
- **Obsługa plików PDF**: Moduł będzie oferował dwa podejścia do plików PDF. W
  pierwszej kolejności spróbuje wyodrębnić tekst bezpośrednio z warstwy
  tekstowej dokumentu (przy użyciu bibliotek takich jak `PyMuPDF` lub
  `pdfplumber`). Jeśli plik okaże się skanem (zawierającym jedynie obrazy),
  system automatycznie skonwertuje strony na format graficzny (np. za pomocą
  `pdf2image`) i podda je pełnemu procesowi OCR.
- **Ekstrakcja metadanych**: Poza samym tekstem, moduł będzie odpowiedzialny za
  wyznaczanie pozycji słów na stronie. Dane te są niezbędne dla modułu
  przeszukiwania, aby mógł on wskazać dokładną lokalizację trafień.

### Cache

Program będzie cachował informacje w plikach ukrytych za pomocą
`.nazwa_pliku.cache`. W plikach tych będzie znajdował się wcześniej
wyekstrachowany tekst, oraz mapa hashująca opisana w kolejnej sekcji.

### Przeszukiwanie tekstu

Podczas Cachowania program będzie też tworzył i zapisywał do pliku mapę
hashująca przyjmującą na klucz słowo, a trzymające częstość oraz koordynaty
wystąpień w pliku danego słowa. Przykład (pseudokod Python):

```py
query = Ala ma Kota
{
    "Ala" : (2, [500, 750])
    "ma" : (35, [7, 20, ....])
    "kota" : (7, [15, 30, ....])
}
```

Algorytm zacznie przeszukiwanie od słowa z najmniejszą ilością wystąpień, czyli
"Ala" i jego pierwszego wystąpienia, jeśli nie znajdzie to przejdzie do
kolejnego wystąpienia "Ala", a potem do "kota".
