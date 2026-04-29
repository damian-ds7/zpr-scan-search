# Instrukcje szkielet

## Generowanie dokumentacji

Projekt korzysta ze standardowych narzędzi Rust do dokumentacji. Aby wygenerować
pełną dokumentację, należy użyć poniższej komendy:

```bash
cargo doc --no-deps --document-private-items --open
```

## Budowanie i środowisko Docker

Wszystkie zależności systemowe (Tesseract OCR, nagłówki Python, kompilator C++)
są skonfigurowane w pliku Dockerfile. Aby zbudować obraz i uruchomić kontener w
trybie interaktywnym:

```bash
docker compose run --build app
```

## Uruchomienie wersji demonstracyjnej

Aby przetestować działanie ekstrakcji na przykładowym pliku PDF, należy użyć
poniższej komendy:

```bash
uv run python/search.py resources/text_and_image.pdf
```
