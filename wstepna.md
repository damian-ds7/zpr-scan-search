# Przeszukowanie skanów za pomocą OCR
## Temat i wstęp
 Tematem projektu jest stworzenie aplikacji pozwalającej odczytac i przeszukać zawartość skanów/pdf-ów.
 Aplikacja będzie miałą trzy główne funkcjonalności:
1. Wczytanie katalogu bądź pliku pdf bądź zdjęcia
2. Ekstrakcja tekstu z pliku oraz cachowanie go
3. Przeszukiwanie pliku

Program będzie składał się z nastepujących komponentów.
1. Bardzo prosty i minimalistyczny frontend terminalowy napisany w Rust
2. Moduł odpowiadający za rozpoznanie języka, oraz ekstrakcję za pomocą tekstu za pomocą OCR w Pythonie
3. Moduł odpowiadający za cachowanie tekstu, poprzez zapis.
4. Moduł odpowiadający za przeszukiwanie tekstu, za pomocą mapy hashującej

## Główne podproblemy i ich wykonanie

### OCR


### Cache
Program będzie cachował informacje w plikach ukrytych za pomocą `.nazwa_pliku.cache`. W plikach tych będzie znajdował się wcześniej wyekstrachowany tekst, oraz mapa hashująca opisana w kolejnej sekcji. 

### Przeszukiwanie tekstu

Podczas Cachowania program będzie też tworzył i zapisywał do pliku mapę hashująca przyjmującą na klucz słowo, a trzymające częstość oraz koordynaty wystąpień w pliku danego słowa. Przykład (pseudokod Python):
```py
query = Ala ma Kota
{
    "Ala" : (2, [500, 750])
    "ma" : (35, [7, 20, ....])
    "kota" : (7, [15, 30, ....])
}
```
Algorytm zacznie przeszukiwanie od słowa z najmniejszą ilością wystąpień, czyli "Ala" i jego pierwszego wystąpienia, jeśli nie znajdzie to przejdzie do kolejnego wystąpienia "Ala", a potem do "kota".

