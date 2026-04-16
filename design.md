# Onboarding

Jeśli ma projekty to przechodzi do dashboarda.

To pierwsze co widzi użytkownik który nie ma projektów:

* Wklejenie URL repozytorium (strona musi wykrywać co to za typ połączenia GitHub, SSH itp)
* Ekran uwierzytelnienia (dla repozytoriów prywatnych, zabezpieczonych)
* Przycisk do weryfikacji połączenia
* Wybór gałęzi / taga / commita, z którego ma być stworzona dokumentacja

Następnie przechodzi do modułu indeksowania.

# Indeskowanie

To nie jest zwykły pasek postępu. Aplikacja musi pokazać użytkownikowi, że "silnik pracuje".

* **Stan: Discovery:** Strona wyświetla licznik: "Znaleziono 452 pliki .rs, .js, .py" prawdopodobnie w trakcie skanowania albo przez podsumowanie.
* **Stan: Parsing (Tree-sitter):**
    * React otrzymuje strumień nazw plików.
    * **UI Detail:** Lista ostatnio zmienionych symboli przewijająca się dynamicznie (zgodnie z tym jak są dodawane).
* **Stan: Failure Recovery:** Jeśli plik ma błąd składni, interfejs nie może "wybuchnąć".
    * **UI:** Ikona ostrzeżenia przy pliku w drzewie: "Błąd parsowania linii 45. Pominięto funkcję X".

Przy dodaniu pierwszego repo, do modułu zarządzania uprawnieniami.

Przy kolejnym repozytorium do modułu widok dokumentacji.

# Widok dokumentacji

Jeśli repozytorium jest puste, czyli nie ma plików które możemy przeczytać, musimy dać informacje o tym że ich nie ma.

## Struktura strony (Three-Column Layout):
1.  **Lewa (Navigation):** Drzewo plików + **Symbol Tree** (lista funkcji/struktur w danym pliku uzyskana z backendu).
2.  **Środek (Documentation):** Dokumentacja którą można przeczytać wraz z strukturami poniżej
3.  **Prawa (Context/Metadata):** "Side-panel" pokazujący dane z backendu


Dla funkcji:
- Sygnatura funkcji = nazwa, jakie ma parametry i co zwraca.

Dla zmiennych:
- Typ zmiennej
- Wartość (jeśli jest stała np. 3.14 z typem float), jak nie to fragment kodu.

Dla klass:
- Moduły nadrzędne (co dziedzicy, parametry generyczne)
- Konstruktory (językach z klasami, w rust nie ma strikte konstruktorów jako struktury więc np. pokazujemy tutaj wszystkie metody które zwracają typ `Self`)

Dla wszystkich:
- Jeśli ma komentarz to ten komentarz też
- Link do źródła kodu (link do gh np. `https://github.com/tfo-dot/dockix/blob/main/backend/src/main.rs#L19`, link do 19 linii w pliku `main.rs`), wersjonowany czyli do odpowiedniej wersji, brancha / taga / commita.

# Zarządzanie projektem

Dla nowego projektu: 
- Najpierw opcja modyfikacji domyślnych ról

Po akceptacji widok dokumentacji.

Normalnie (czyli użytkownik wchodzi w to z własnej woli):

Widok administracyjny:
- Lista użytkowników wraz z opcją zmiany ról albo pojedynczych uprawnień
- Lista ról z opcją zmiany ich uprawnień i szczegółów roli (nazwa / opis)
- Podgląd zdarzeń co się stało w projekcie

Tryb podglądu:
- Wyświetlanie projektu w roli
- W trybie podglądu musi widnieć informacja o tym że w nim jesteśmy i opcja wyjścia.
- Zablokowanie edycji w trybie podglądu

# Dashboard użytkownika

Lista projektów ze statusami:
- Nazwa
- Kto jest dostawcą (github, gitlab selfhost)
- Z czego korzysta (branch / commit / tag)
- Status synchronizacji (pełny, w trakcie, błąd, przestarzałe)
- Szybkie akcje, "zobacz dokumentacje", "odświerz", "ustawienia" (raczej jako ikonki niż tekst)

# Dashboard administratora

Statystyki:
- Ile projektów dokumentujemy
- Ile pamięci zajmują na dysku (łącznie)

Logi (ostatnie 10 akcji):
- Kto
- Kiedy
- Co

Statusy projektów