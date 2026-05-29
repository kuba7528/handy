# Handy

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/invite/WVBeWsNXK4)

**Darmowa, otwartoźródłowa i rozszerzalna aplikacja mowa-na-tekst, działająca w pełni offline.**

Handy to wieloplatformowa aplikacja desktopowa zapewniająca prostą, dbającą o prywatność transkrypcję mowy. Naciśnij skrót klawiszowy, mów, a Twoje słowa pojawią się w dowolnym polu tekstowym — wszystko lokalnie na Twoim komputerze, bez wysyłania danych do chmury.

> **Ten fork** ([kuba7528/handy](https://github.com/kuba7528/handy)) rozszerza oryginalne [Handy](https://github.com/cjpais/Handy) o funkcję **ciągłego nasłuchiwania** — aplikacja może automatycznie nasłuchiwać mikrofonu i transkrybować wykryte wypowiedzi bez konieczności wciskania skrótu klawiszowego.

## Ciągłe nasłuchiwanie

Ta wersja repozytorium dodaje tryb **ciągłego nasłuchiwania**, który ułatwia dyktowanie bez ciągłego trzymania skrótu:

1. Włącz opcję w **Ustawienia → Debugowanie → Ciągłe nasłuchiwanie** (domyślnie włączone w tym forku).
2. Aplikacja otwiera mikrofon i nasłuchuje w tle aż do zamknięcia programu lub wyłączenia funkcji.
3. Wykryte segmenty mowy są automatycznie transkrybowane i wklejane do aktywnego pola tekstowego.
4. Ręczna transkrypcja skrótem klawiszowym pozostaje dostępna, gdy ciągłe nasłuchiwanie jest wyłączone.

Funkcja wykorzystuje detekcję aktywności głosowej (VAD), aby rozpoznawać początek i koniec wypowiedzi, a następnie przetwarza je lokalnie modelem Whisper lub Parakeet.

## Dlaczego Handy?

Handy powstało, aby wypełnić lukę po naprawdę otwartoźródłowym i rozszerzalnym narzędziu mowa-na-tekst. Jak podkreśla [handy.computer](https://handy.computer):

- **Darmowe**: narzędzia dostępności powinny być w rękach wszystkich, a nie za paywallem
- **Otwarte źródło**: razem możemy budować dalej — rozszerzaj Handy dla siebie i współtwórz coś większego
- **Prywatne**: Twój głos zostaje na komputerze — transkrypcja bez wysyłania audio do chmury
- **Proste**: jedno narzędzie, jedno zadanie — transkrybuj to, co mówisz, i wklejaj do pola tekstowego

Handy nie stara się być najlepszą aplikacją mowa-na-tekst — stara się być najbardziej „forkowalną”.

## Jak to działa

### Tryb skrótu klawiszowego (domyślny w oryginalnym Handy)

1. **Naciśnij** konfigurowalny skrót klawiszowy, aby rozpocząć/zatrzymać nagrywanie (lub użyj trybu push-to-talk)
2. **Mów**, gdy skrót jest aktywny
3. **Puść** skrót — Handy przetwarza mowę za pomocą Whisper
4. **Otrzymaj** transkrybowany tekst wklejony bezpośrednio do aplikacji, której używasz

### Tryb ciągłego nasłuchiwania (ten fork)

1. **Włącz** ciągłe nasłuchiwanie w ustawieniach
2. **Mów** naturalnie — aplikacja wykrywa wypowiedzi automatycznie
3. **Otrzymaj** transkrypcje w miarę kończenia segmentów mowy

Cały proces odbywa się lokalnie:

- Cisza jest filtrowana przez VAD (Voice Activity Detection) z Silero
- Transkrypcja korzysta z wybranego modelu:
  - **Modele Whisper** (Small/Medium/Turbo/Large) z akceleracją GPU, gdy dostępna
  - **Parakeet V3** — model zoptymalizowany pod CPU, z doskonałą wydajnością i automatycznym wykrywaniem języka
- Działa na Windows, macOS i Linux

## Szybki start

### Instalacja

1. Pobierz najnowsze wydanie ze [strony releases](https://github.com/kuba7528/handy/releases) lub zbuduj ze źródeł (patrz poniżej)
   - **macOS**: dostępne także przez [Homebrew cask](https://formulae.brew.sh/cask/handy): `brew install --cask handy`
   - **Windows**: dostępne także przez [winget](https://github.com/microsoft/winget-pkgs): `winget install cjpais.Handy` \
     **Uwaga:** pakiet Homebrew cask i winget nie są utrzymywane przez deweloperów Handy.
2. Zainstaluj aplikację
3. Uruchom Handy i nadaj wymagane uprawnienia systemowe (mikrofon, dostępność)
4. Skonfiguruj preferowane skróty klawiszowe w Ustawieniach
5. (Opcjonalnie) Włącz **Ciągłe nasłuchiwanie** w Ustawienia → Debugowanie
6. Zacznij transkrybować!

### Budowanie ze źródeł

**Wymagania:** [Rust](https://rustup.rs/), [Bun](https://bun.sh/), [zależności Tauri](https://tauri.app/start/prerequisites/)

```bash
git clone https://github.com/kuba7528/handy.git
cd handy
bun install
bun tauri dev          # tryb deweloperski
bun run tauri build    # build produkcyjny
```

Szczegółowe instrukcje budowania, w tym wymagania specyficzne dla platform, znajdują się w [BUILD.md](BUILD.md).

**Windows (skrypt pomocniczy):**

```powershell
.\build-windows.ps1
```

## Integracje

<a href="https://www.raycast.com/mattiacolombomc/handy" title="Install Handy Raycast Extension"><img src="https://www.raycast.com/mattiacolombomc/handy/install_button@2x.png?v=1.1" height="64" style="height: 64px;" alt="Install handy Raycast Extension" /></a>

Steruj Handy z poziomu [Raycast](https://www.raycast.com) — uruchamiaj/zatrzymuj nagrywanie, przeglądaj historię transkrypcji, zarządzaj słownikiem, przełączaj modele i języki.

[Źródło](https://github.com/mattiacolombomc/raycast-handy) · autor: [@mattiacolombomc](https://github.com/mattiacolombomc)

## Architektura

Handy jest zbudowane jako aplikacja Tauri łącząca:

- **Frontend**: React + TypeScript z Tailwind CSS dla interfejsu ustawień
- **Backend**: Rust do integracji systemowej, przetwarzania audio i inferencji ML
- **Kluczowe biblioteki**:
  - `whisper-rs`: lokalne rozpoznawanie mowy z modelami Whisper
  - `transcribe-rs`: rozpoznawanie mowy zoptymalizowane pod CPU z modelami Parakeet
  - `cpal`: wieloplatformowe I/O audio
  - `vad-rs`: detekcja aktywności głosowej (VAD)
  - `rdev`: globalne skróty klawiszowe i zdarzenia systemowe
  - `rubato`: resampling audio

### Tryb debugowania

Handy zawiera zaawansowany tryb debugowania do rozwoju i rozwiązywania problemów. Dostęp:

- **macOS**: `Cmd+Shift+D`
- **Windows/Linux**: `Ctrl+Shift+D`

### Parametry CLI

Handy obsługuje flagi wiersza poleceń do sterowania działającą instancją i dostosowywania startu. Działają na wszystkich platformach (macOS, Windows, Linux).

**Flagi zdalnego sterowania** (wysyłane do już działającej instancji przez plugin single-instance):

```bash
handy --toggle-transcription    # Przełącz nagrywanie wł/wył
handy --toggle-post-process     # Przełącz nagrywanie z postprocesem wł/wył
handy --cancel                  # Anuluj bieżącą operację
```

**Flagi startu:**

```bash
handy --start-hidden            # Uruchom bez pokazywania głównego okna
handy --no-tray                 # Uruchom bez ikony w zasobniku systemowym
handy --debug                   # Włącz tryb debugowania z pełnym logowaniem
handy --help                    # Pokaż wszystkie dostępne flagi
```

Flagi można łączyć, np. przy autostarcie:

```bash
handy --start-hidden --no-tray
```

> **Wskazówka dla macOS:** Gdy Handy jest zainstalowane jako pakiet .app, wywołuj binarkę bezpośrednio:
>
> ```bash
> /Applications/Handy.app/Contents/MacOS/Handy --toggle-transcription
> ```

## Znane problemy i ograniczenia

Projekt jest aktywnie rozwijany i ma [znane problemy](https://github.com/cjpais/Handy/issues). Wierzymy w transparentność co do obecnego stanu:

### Główne problemy (szukamy pomocy)

**Awarię modeli Whisper:**

- Modele Whisper mogą się wyłączać na niektórych konfiguracjach systemowych (Windows i Linux)
- Nie dotyczy wszystkich systemów — problem zależy od konfiguracji
  - Jeśli doświadczasz awarii i jesteś deweloperem, pomóż naprawić problem i dostarcz logi debugowania!

**Obsługa Wayland (Linux):**

- Ograniczona obsługa serwera wyświetlania Wayland
- Wymaga [`wtype`](https://github.com/atx/wtype) lub [`dotool`](https://sr.ht/~geb/dotool/) do poprawnego wklejania tekstu (patrz [Uwagi dla Linuxa](#uwagi-dla-linuxa) poniżej)

### Uwagi dla Linuxa

**Narzędzia do wprowadzania tekstu:**

Aby niezawodnie wklejać tekst na Linuxie, zainstaluj odpowiednie narzędzie dla swojego serwera wyświetlania:

| Serwer wyświetlania | Zalecane narzędzie | Polecenie instalacji                               |
| ------------------- | ------------------ | -------------------------------------------------- |
| X11                 | `xdotool`          | `sudo apt install xdotool`                         |
| Wayland             | `wtype`            | `sudo apt install wtype`                           |
| Oba                 | `dotool`           | `sudo apt install dotool` (wymaga grupy `input`)   |

- **X11**: zainstaluj `xdotool` do bezpośredniego wpisywania i wklejania ze schowka
- **Wayland**: zainstaluj `wtype` (preferowane) lub `dotool` do poprawnego wprowadzania tekstu
- **Konfiguracja dotool**: wymaga dodania użytkownika do grupy `input`: `sudo usermod -aG input $USER` (następnie wyloguj się i zaloguj ponownie)

Bez tych narzędzi Handy korzysta z enigo, co może mieć ograniczoną kompatybilność, zwłaszcza na Wayland.

**Inne uwagi:**

- **Zależność biblioteki runtime (`libgtk-layer-shell.so.0`)**:
  - Handy linkuje `gtk-layer-shell` na Linuxie. Jeśli start kończy się błędem `error while loading shared libraries: libgtk-layer-shell.so.0`, zainstaluj pakiet runtime dla swojej dystrybucji:

    | Dystrybucja   | Pakiet do instalacji  | Przykładowe polecenie                  |
    | ------------- | --------------------- | -------------------------------------- |
    | Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
    | Fedora/RHEL   | `gtk-layer-shell`     | `sudo dnf install gtk-layer-shell`     |
    | Arch Linux    | `gtk-layer-shell`     | `sudo pacman -S gtk-layer-shell`       |

  - Przy budowaniu ze źródeł na Ubuntu/Debian może być też potrzebny `libgtk-layer-shell-dev`.

- Nakładka nagrywania jest domyślnie wyłączona na Linuxie (`Pozycja nakładki: Brak`), ponieważ niektóre kompository traktują ją jak aktywne okno. Gdy nakładka jest widoczna, może przejąć fokus i uniemożliwić wklejenie tekstu do aplikacji, która wywołała transkrypcję. Jeśli mimo to włączysz nakładkę, wklejanie ze schowka może się nie powieść lub trafić do niewłaściwego okna.
- Przy problemach z aplikacją pomocne może być uruchomienie ze zmienną środowiskową `WEBKIT_DISABLE_DMABUF_RENDERER=1`
- Jeśli Handy nie startuje niezawodnie na Linuxie, patrz [Rozwiązywanie problemów → Awarie lub niestabilność startu na Linuxie](#awarie-lub-niestabilność-startu-na-linuxie).
- **Globalne skróty klawiszowe (Wayland):** Na Wayland skróty systemowe konfiguruje się przez środowisko pulpitu lub menedżer okien. Użyj [flag CLI](#parametry-cli) jako polecenia dla własnego skrótu.

  **GNOME:**
  1. Otwórz **Ustawienia > Klawiatura > Skróty klawiszowe > Własne skróty**
  2. Kliknij **+**, aby dodać nowy skrót
  3. Ustaw **Nazwę** na `Przełącz transkrypcję Handy`
  4. Ustaw **Polecenie** na `handy --toggle-transcription`
  5. Kliknij **Ustaw skrót** i naciśnij kombinację klawiszy (np. `Super+O`)

  **KDE Plasma:**
  1. Otwórz **Ustawienia systemu > Skróty > Własne skróty**
  2. Kliknij **Edytuj > Nowy > Skrót globalny > Polecenie/URL**
  3. Nazwij go `Przełącz transkrypcję Handy`
  4. W zakładce **Wyzwalacz** ustaw kombinację klawiszy
  5. W zakładce **Akcja** ustaw polecenie `handy --toggle-transcription`

  **Sway / i3:**

  Dodaj do pliku konfiguracyjnego (`~/.config/sway/config` lub `~/.config/i3/config`):

  ```ini
  bindsym $mod+o exec handy --toggle-transcription
  ```

  **Hyprland:**

  Dodaj do pliku konfiguracyjnego (`~/.config/hypr/hyprland.conf`):

  ```ini
  bind = $mainMod, O, exec, handy --toggle-transcription
  ```

- Globalne skróty można też obsługiwać poza Handy przez sygnały Unix, co pozwala menedżerom okien Wayland lub daemonom hotkey zachować własność skrótów:

  | Sygnał    | Akcja                                      | Przykład               |
  | --------- | ------------------------------------------ | ---------------------- |
  | `SIGUSR2` | Przełącz transkrypcję                        | `pkill -USR2 -n handy` |
  | `SIGUSR1` | Przełącz transkrypcję z postprocesem       | `pkill -USR1 -n handy` |

  Przykład konfiguracji Sway:

  ```ini
  bindsym $mod+o exec pkill -USR2 -n handy
  bindsym $mod+p exec pkill -USR1 -n handy
  ```

  `pkill` tutaj tylko dostarcza sygnał — nie kończy procesu.

**Problemy z nakładką i wklejaniem (Linux):**

- Okno nakładki nagrywania może zakłócać wklejanie transkrybowanego tekstu do docelowych aplikacji na Linuxie (X11)
- **Rozwiązanie:** Otwórz **Ustawienia > Zaawansowane** i ustaw **„Pozycja nakładki”** na **„Brak”**, aby wyłączyć nakładkę
- Włącz **„Informacja dźwiękowa”** (także w Zaawansowanych), jeśli nadal chcesz słyszeć potwierdzenie stanu nagrywania
- Użytkownicy aktualizujący ze starszych wersji lub importujący ustawienia z innych platform mogą musieć ręcznie zastosować tę zmianę

### Obsługiwane platformy

- **macOS** (Intel i Apple Silicon)
- **Windows x64**
- **Linux x64**

### Wymagania systemowe / zalecenia

Poniżej zalecenia dotyczące uruchamiania Handy na własnym komputerze. Jeśli nie spełniasz wymagań, wydajność aplikacji może być obniżona. Pracujemy nad poprawą wydajności na różnych konfiguracjach sprzętowych.

**Dla modeli Whisper:**

- **macOS**: Mac z serią M, Mac Intel
- **Windows**: GPU Intel, AMD lub NVIDIA
- **Linux**: GPU Intel, AMD lub NVIDIA
  - Ubuntu 22.04, 24.04

**Dla modelu Parakeet V3:**

- **Tylko CPU** — działa na szerokiej gamie sprzętu
- **Minimum**: Intel Skylake (6. gen) lub równoważne procesory AMD
- **Wydajność**: ~5× szybciej niż czas rzeczywisty na sprzęcie średniej klasy (testowane na i5)
- **Automatyczne wykrywanie języka** — bez ręcznego wyboru języka

## Plan rozwoju i aktywny development

Aktywnie pracujemy nad wieloma funkcjami i ulepszeniami. Wkład i opinie są mile widziane!

### W trakcie prac

**Logowanie debugowania:**

- Dodawanie logów debugowania do pliku w celu diagnozowania problemów

**Ulepszenia klawiatury na macOS:**

- Obsługa klawisza Globe jako wyzwalacza transkrypcji
- Przepisanie obsługi globalnych skrótów dla macOS i potencjalnie innych systemów

**Analityka opcjonalna:**

- Zbieranie anonimowych danych użycia w celu ulepszania Handy
- Podejście zorientowane na prywatność z wyraźną zgodą użytkownika

**Refaktoryzacja ustawień:**

- Porządkowanie i refaktoryzacja systemu ustawień, który staje się rozbudowany
- Lepsze abstrakcje zarządzania ustawieniami

**Uporządkowanie poleceń Tauri:**

- Abstrakcja i organizacja wzorców poleceń Tauri
- Badanie tauri-specta dla lepszej type safety i organizacji

## Weryfikacja podpisów wydań

Artefakty wydań Handy są podpisywane w formacie podpisu aktualizatora Tauri. Klucz publiczny znajduje się w [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) pod `plugins.updater.pubkey`.

Aby ręcznie zweryfikować wydanie, ustaw `ARTIFACT` na nazwę pobranego pliku, zapisz wartość `pubkey` z `src-tauri/tauri.conf.json` do `handy.pub.b64`, następnie zdekoduj klucz publiczny i plik `.sig` z base64 i zweryfikuj artefakt za pomocą `minisign`:

```bash
# Zamień na pobrany plik
ARTIFACT="Handy_0.8.1_amd64.AppImage"

python3 - "$ARTIFACT" <<'PY'
import base64, pathlib, sys

artifact = sys.argv[1]

pub = pathlib.Path("handy.pub.b64").read_text().strip()
pathlib.Path("handy.pub").write_bytes(base64.b64decode(pub))

sig = pathlib.Path(f"{artifact}.sig").read_text().strip()
pathlib.Path(f"{artifact}.minisig").write_bytes(base64.b64decode(sig))
PY

minisign -Vm "$ARTIFACT" \
  -p handy.pub \
  -x "$ARTIFACT.minisig"
```

Po sukcesie `minisign` wyświetli:

```text
Signature and comment signature verified
```

Nie używaj `gpg` do plików `.sig`.

## Rozwiązywanie problemów

### Ręczna instalacja modeli (proxy / ograniczenia sieci)

Jeśli jesteś za proxy, firewallem lub w ograniczonym środowisku sieciowym, gdzie Handy nie może automatycznie pobrać modeli, możesz zainstalować je ręcznie. Adresy URL są publicznie dostępne z dowolnej przeglądarki.

#### Krok 1: Znajdź katalog danych aplikacji

1. Otwórz ustawienia Handy
2. Przejdź do sekcji **O programie**
3. Skopiuj ścieżkę „Katalog danych aplikacji” lub użyj skrótów:
   - **macOS**: `Cmd+Shift+D` — menu debugowania
   - **Windows/Linux**: `Ctrl+Shift+D` — menu debugowania

Typowe ścieżki:

- **macOS**: `~/Library/Application Support/com.pais.handy/`
- **Windows**: `C:\Users\{username}\AppData\Roaming\com.pais.handy\`
- **Linux**: `~/.config/com.pais.handy/`

#### Krok 2: Utwórz katalog modeli

W katalogu danych aplikacji utwórz folder `models`, jeśli jeszcze nie istnieje:

```bash
# macOS/Linux
mkdir -p ~/Library/Application\ Support/com.pais.handy/models

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:APPDATA\com.pais.handy\models"
```

#### Krok 3: Pobierz pliki modeli

Pobierz potrzebne modele z poniższych adresów.

**Modele Whisper (pojedyncze pliki .bin):**

- Small (487 MB): `https://blob.handy.computer/ggml-small.bin`
- Medium (492 MB): `https://blob.handy.computer/whisper-medium-q4_1.bin`
- Turbo (1600 MB): `https://blob.handy.computer/ggml-large-v3-turbo.bin`
- Large (1100 MB): `https://blob.handy.computer/ggml-large-v3-q5_0.bin`

**Modele Parakeet (archiwa skompresowane):**

- V2 (473 MB): `https://blob.handy.computer/parakeet-v2-int8.tar.gz`
- V3 (478 MB): `https://blob.handy.computer/parakeet-v3-int8.tar.gz`

#### Krok 4: Zainstaluj modele

**Modele Whisper (pliki .bin):**

Umieść plik `.bin` bezpośrednio w katalogu `models`:

```
{app_data_dir}/models/
├── ggml-small.bin
├── whisper-medium-q4_1.bin
├── ggml-large-v3-turbo.bin
└── ggml-large-v3-q5_0.bin
```

**Modele Parakeet (archiwa .tar.gz):**

1. Rozpakuj plik `.tar.gz`
2. Umieść **rozpakowany katalog** w folderze `models`
3. Katalog musi mieć dokładnie taką nazwę:
   - **Parakeet V2**: `parakeet-tdt-0.6b-v2-int8`
   - **Parakeet V3**: `parakeet-tdt-0.6b-v3-int8`

Docelowa struktura:

```
{app_data_dir}/models/
├── parakeet-tdt-0.6b-v2-int8/     (katalog z plikami modelu)
│   ├── (pliki modelu)
│   └── (pliki konfiguracyjne)
└── parakeet-tdt-0.6b-v3-int8/     (katalog z plikami modelu)
    ├── (pliki modelu)
    └── (pliki konfiguracyjne)
```

**Ważne uwagi:**

- Dla modeli Parakeet nazwa rozpakowanego katalogu **musi** dokładnie odpowiadać powyższym
- Nie zmieniaj nazw plików `.bin` dla modeli Whisper — używaj dokładnych nazw z adresów pobierania
- Po umieszczeniu plików uruchom ponownie Handy, aby wykryło nowe modele

#### Krok 5: Zweryfikuj instalację

1. Uruchom ponownie Handy
2. Otwórz Ustawienia → Modele
3. Ręcznie zainstalowane modele powinny pojawić się jako „Pobrane”
4. Wybierz model i przetestuj transkrypcję

### Własne modele Whisper

Handy może automatycznie wykrywać własne modele Whisper GGML umieszczone w katalogu `models`. Przydatne dla użytkowników chcących korzystać z modeli fine-tuned lub społecznościowych spoza domyślnej listy.

**Jak używać:**

1. Uzyskaj model Whisper w formacie GGML `.bin` (np. z [Hugging Face](https://huggingface.co/models?search=whisper%20ggml))
2. Umieść plik `.bin` w katalogu `models` (patrz ścieżki powyżej)
3. Uruchom ponownie Handy, aby wykryło nowy model
4. Model pojawi się w sekcji „Własne modele” na stronie ustawień modeli

**Ważne:**

- Modele społecznościowe są dostarczane przez użytkowników i mogą nie mieć wsparcia przy rozwiązywaniu problemów
- Model musi być poprawnym formatem Whisper GGML (plik `.bin`)
- Nazwa modelu pochodzi z nazwy pliku (np. `my-custom-model.bin` → „My Custom Model”)

### Awarie lub niestabilność startu na Linuxie

Jeśli Handy nie startuje niezawodnie na Linuxie — np. zamyka się zaraz po uruchomieniu, nie pokazuje okna lub zgłasza błąd protokołu Wayland — wypróbuj poniższe kroki po kolei.

**1. Zainstaluj (lub przeinstaluj) `gtk-layer-shell`**

Handy używa `gtk-layer-shell` dla nakładki nagrywania i linkuje się z nią w runtime. Brakująca lub uszkodzona instalacja to najczęstsza przyczyna problemów ze startem. Upewnij się, że pakiet runtime jest zainstalowany:

| Dystrybucja   | Pakiet do instalacji  | Przykładowe polecenie                  |
| ------------- | --------------------- | -------------------------------------- |
| Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
| Fedora/RHEL   | `gtk-layer-shell`     | `sudo dnf install gtk-layer-shell`     |
| Arch Linux    | `gtk-layer-shell`     | `sudo pacman -S gtk-layer-shell`       |

Jeśli jest zainstalowany, a problemy trwają, spróbuj przeinstalować (np. `sudo pacman -S gtk-layer-shell`), gdy pliki biblioteki mogły ulec uszkodzeniu przy częściowej aktualizacji.

**2. Wyłącz nakładkę GTK layer shell (`HANDY_NO_GTK_LAYER_SHELL`)**

Jeśli instalacja biblioteki nie pomaga, możesz całkowicie pominąć inicjalizację `gtk-layer-shell`. Na niektórych compositorach (np. KDE Plasma pod Wayland) zgłaszano złe interakcje z nakładką nagrywania. Przy tej zmiennej nakładka używa zwykłego okna always-on-top:

```bash
HANDY_NO_GTK_LAYER_SHELL=1 handy
```

**3. Wyłącz renderer WebKit DMA-BUF (`WEBKIT_DISABLE_DMABUF_RENDERER`)**

Na niektórych kombinacjach GPU/sterowników renderer WebKitGTK DMA-BUF może powodować brak renderowania okna lub awarię. Spróbuj:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 handy
```

**Trwałe zastosowanie obejścia**

Gdy znajdziesz działającą flagę, wyeksportuj ją w profilu powłoki (`~/.bashrc`, `~/.zshenv`, …) lub w wpisie autostartu pulpitu. Przy uruchamianiu z pliku `.desktop` możesz dodać prefiks do linii `Exec=`, np.:

```ini
Exec=env HANDY_NO_GTK_LAYER_SHELL=1 handy
```

Jeśli obejście pomogło, [zgłoś issue](https://github.com/cjpais/Handy/issues) z opisem dystrybucji, środowiska pulpitu i typu sesji — pomoże to zawęzić źródło błędu.

### Jak współtworzyć

1. **Sprawdź istniejące zgłoszenia** na [github.com/cjpais/Handy/issues](https://github.com/cjpais/Handy/issues)
2. **Sforkuj repozytorium** i utwórz branch funkcji
3. **Testuj dokładnie** na docelowej platformie
4. **Wyślij pull request** z jasnym opisem zmian
5. **Dołącz do dyskusji** — napisz na [contact@handy.computer](mailto:contact@handy.computer)

Celem jest stworzenie zarówno użytecznego narzędzia, jak i fundamentu do dalszego rozwoju — przejrzystej, prostej bazy kodu służącej społeczności.

## Sponsorzy

<div align="center">
  Jesteśmy wdzięczni sponsorom, którzy pomagają uczynić Handy możliwym:
  <br><br>
  <a href="https://wordcab.com">
    <img src="sponsor-images/wordcab.png" alt="Wordcab" width="120" height="120">
  </a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://github.com/epicenter-so/epicenter">
    <img src="sponsor-images/epicenter.png" alt="Epicenter" width="120" height="120">
  </a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://boltai.com?utm_source=handy">
    <img src="sponsor-images/boltai.jpg" alt="Bolt AI" width="120" height="120">
  </a>
</div>

## Powiązane projekty

- **[Handy CLI](https://github.com/cjpais/handy-cli)** — oryginalna wersja wiersza poleceń w Pythonie
- **[handy.computer](https://handy.computer)** — strona projektu z demo i dokumentacją
- **[cjpais/Handy](https://github.com/cjpais/Handy)** — upstream, od którego pochodzi ten fork

## Licencja

Licencja MIT — szczegóły w pliku [LICENSE](LICENSE).

## Podziękowania

- **Whisper** (OpenAI) za model rozpoznawania mowy
- **whisper.cpp i ggml** za wieloplatformową inferencję/accelerację Whisper
- **Silero** za lekki i skuteczny VAD
- Zespół **Tauri** za doskonały framework aplikacji w Rust
- **Współtwórcy społeczności**, którzy pomagają ulepszać Handy
